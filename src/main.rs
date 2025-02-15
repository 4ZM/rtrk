mod interaction;
mod pos;
mod runtime;
mod term;
mod widget;

mod app {
    use crate::impl_focusable_with_focuschain;
    use crate::interaction::Event;
    use crate::pos::Pos;
    use crate::spinner;
    use crate::spinner::{spinner_rc, SpinnerRc, SpinnerView};
    use crate::widget::button::{button_rc, ButtonRc, ButtonView};
    use crate::widget::focus::{FocusChain, FocusableRc};
    use crate::widget::label::{label, Label};
    use crate::widget::textbox;
    use crate::widget::textbox::{textbox_rc, TextBoxRc, TextBoxView};
    use crate::widget::{Focusable, View, Widget};

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        Reset,
        NextFocus,
        Spinner(usize, spinner::Message),
        Text(textbox::Message),
    }

    pub struct App {
        spin: Vec<SpinnerRc>,
        focus_chain: FocusChain,
        reset_btn: ButtonRc<Message>,
        txt: TextBoxRc,
        n_from_txt: Option<i64>,
    }

    impl App {
        pub fn new() -> Self {
            let initial_values = vec![23, 42];
            let initial_sum = initial_values.iter().sum();
            let spin: Vec<SpinnerRc> = initial_values
                .iter()
                .map(|x| spinner_rc(initial_sum, *x))
                .collect();
            let reset_btn = button_rc("RST", Message::Reset);

            let txt = textbox_rc(10);

            let mut focus_chain = FocusChain::new();
            focus_chain.push(txt.clone() as FocusableRc);
            for s in &spin {
                focus_chain.push(s.clone() as FocusableRc);
            }
            focus_chain.push(reset_btn.clone() as FocusableRc);

            Self {
                focus_chain,
                spin,
                reset_btn,
                txt,
                n_from_txt: None,
            }
        }

        fn sum(&self) -> i64 {
            self.spin.iter().map(|s| s.borrow().value()).sum()
        }
    }
    impl Widget<Message, AppView> for App {
        fn update(&mut self, msg: Message) {
            match msg {
                Message::Spinner(i, msg) => {
                    self.spin[i].borrow_mut().update(msg);

                    // NB: sum() will borrow spinners and can't be called
                    // when we have a mutable borrow in the loop. Get the sum first.
                    let sum = self.sum();

                    for s in self.spin.iter() {
                        s.borrow_mut().update(spinner::Message::SumChanged(sum))
                    }
                }
                Message::NextFocus => self.next_focus(),
                Message::Reset => {
                    for s in self.spin.iter() {
                        s.borrow_mut().update(spinner::Message::SetValue(0)); //self.spin.iter().for_each(|s| s.borrow_mut().value = 0),
                    }
                }
                Message::Text(m) => {
                    self.txt.borrow_mut().update(m);

                    self.n_from_txt = match self.txt.borrow().text().trim().parse::<i64>() {
                        Ok(n) if n >= 0 && n < 256 => Some(n),
                        _ => None,
                    }
                }
            };
        }

        fn view(&self, pos: Pos) -> AppView {
            let hex_str = match self.n_from_txt {
                Some(n) => format!("0x{:02x}", n),
                _ => "----".to_string(),
            };

            AppView {
                txt: self.txt.borrow().view(pos + Pos { r: 10, c: 9 }),
                hex_lbl: label(pos + Pos { r: 12, c: 10 }, &hex_str),
                sum_lbl: label(pos + Pos { r: 7, c: 10 }, &format!("SUM: {}", self.sum())),
                rst_btn: self.reset_btn.borrow().view(pos + Pos { r: 6, c: 9 }),
                spinners: self
                    .spin
                    .iter()
                    .enumerate()
                    .map(|(i, s)| {
                        s.borrow().view(
                            pos + Pos {
                                r: 0,
                                c: i as u16 * 10, // Horizontally spaced
                            },
                        )
                    })
                    .collect(),
            }
        }
    }
    impl_focusable_with_focuschain!(App, focus_chain);

    pub struct AppView {
        txt: TextBoxView,
        rst_btn: ButtonView<Message>,
        sum_lbl: Label,
        hex_lbl: Label,
        spinners: Vec<SpinnerView>,
    }
    impl View<Message> for AppView {
        fn draw(&self, renderer: &mut dyn crate::interaction::Renderer) {
            // TODO: Children can be used generically here
            for s in self.spinners.iter() {
                s.draw(renderer);
            }

            self.txt.draw(renderer);
            self.rst_btn.draw(renderer);
            self.sum_lbl.draw(renderer);
            self.hex_lbl.draw(renderer);
        }
        fn on_event(&self, e: Event) -> Vec<Message> {
            if let Event::NextFocus = e {
                return vec![Message::NextFocus];
            }

            // TODO children() can be used generically here. Or can it? Spinner message is not generic
            // could have a ChildMessage routed by child ID or similar
            let mut msgs: Vec<Message> = vec![];
            for (i, s) in self.spinners.iter().enumerate() {
                s.on_event(e)
                    .iter()
                    .for_each(|&m| msgs.push(Message::Spinner(i, m)));
            }

            self.rst_btn.on_event(e).iter().for_each(|&m| msgs.push(m));
            self.txt
                .on_event(e)
                .iter()
                .for_each(|&m| msgs.push(Message::Text(m)));

            msgs
        }
    }
}

mod spinner {
    use crate::impl_focusable_with_focuschain;
    use crate::interaction::Event;
    use crate::pos::Pos;
    use crate::widget::button::{button_rc, ButtonRc, ButtonView};
    use crate::widget::focus::{FocusChain, FocusableRc};
    use crate::widget::label::{label, Label};
    use crate::widget::{Focusable, View, Widget};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        NextFocus,
        Increment,
        Decrement,
        SumChanged(i64),
        SetValue(i64),
    }

    pub struct Spinner {
        global_sum: i64, // Or could use fraction here?
        value: i64,
        inc_btn: ButtonRc<Message>,
        dec_btn: ButtonRc<Message>,
        focus_chain: FocusChain,
    }
    impl Spinner {
        pub fn new(global_sum: i64, initial_value: i64) -> Self {
            let inc_btn = button_rc("+", Message::Increment);
            let dec_btn = button_rc("-", Message::Decrement);

            let mut focus_chain = FocusChain::new();
            focus_chain.push(inc_btn.clone() as FocusableRc);
            focus_chain.push(dec_btn.clone() as FocusableRc);

            Self {
                value: initial_value,
                global_sum,
                inc_btn,
                dec_btn,
                focus_chain,
            }
        }
        pub fn value(&self) -> i64 {
            self.value
        }
    }
    impl Widget<Message, SpinnerView> for Spinner {
        fn update(&mut self, msg: Message) {
            match msg {
                Message::Increment => self.value += 1,
                Message::Decrement => self.value -= 1,
                Message::NextFocus => self.next_focus(),
                Message::SetValue(v) => self.value = v,
                Message::SumChanged(s) => self.global_sum = s,
            }
        }

        fn view(&self, pos: Pos) -> SpinnerView {
            let lbl = match self.global_sum {
                0 => format!("{} (-)", self.value), // Avoid div zero
                sum => {
                    let fraction = self.value as f32 / sum as f32;
                    format!("{} ({:.0})", self.value, fraction * 100f32)
                }
            };

            SpinnerView {
                lbl: label(pos + Pos { r: 1, c: 1 }, &lbl),
                inc_btn: self.inc_btn.borrow().view(pos + Pos { r: 0, c: 0 }),
                dec_btn: self.dec_btn.borrow().view(pos + Pos { r: 2, c: 0 }),
            }
        }
    }

    impl_focusable_with_focuschain!(Spinner, focus_chain);

    pub struct SpinnerView {
        inc_btn: ButtonView<Message>,
        dec_btn: ButtonView<Message>,
        lbl: Label,
    }
    impl View<Message> for SpinnerView {
        fn draw(&self, renderer: &mut dyn crate::interaction::Renderer) {
            self.inc_btn.draw(renderer);
            self.lbl.draw(renderer);
            self.dec_btn.draw(renderer);
        }
        fn on_event(&self, e: Event) -> Vec<Message> {
            let focus_msg = match e {
                Event::NextFocus => vec![Message::NextFocus],
                _ => vec![],
            };

            vec![
                focus_msg,
                self.inc_btn.on_event(e),
                self.dec_btn.on_event(e),
            ]
            .concat()
        }
    }

    pub type SpinnerRc = Rc<RefCell<Spinner>>;
    pub fn spinner_rc(sum: i64, value: i64) -> SpinnerRc {
        Rc::new(RefCell::new(Spinner::new(sum, value)))
    }
}

fn main() {
    let mut app = app::App::new();
    runtime::start(&mut app);
}

#[cfg(test)]
mod tests {

    use super::interaction::tests::TestRenderer;
    use super::pos::*;
    use super::widget::*;
    use super::*;

    #[test]
    fn spinner_state_update_test() {
        let mut app = spinner::Spinner::new(0, 0);
        assert_eq!(app.value(), 0);

        let _ = app.update(spinner::Message::Increment);
        app.update(spinner::Message::Increment);
        app.update(spinner::Message::Decrement);

        assert_eq!(app.value(), 1);
    }

    #[test]
    fn spinner_rendering_test() {
        let mut app = spinner::Spinner::new(0, 0);
        assert_eq!(app.value(), 0);

        let view = app.view(Pos { r: 0, c: 0 });
        let mut renderer = TestRenderer::new();
        view.draw(&mut renderer);
        assert_eq!(renderer.out, " + 0 (-) - ");

        app.update(spinner::Message::NextFocus);
        let view = app.view(Pos { r: 0, c: 0 });
        let mut renderer = TestRenderer::new();
        view.draw(&mut renderer);
        assert_eq!(renderer.out, "[+]0 (-) - ");
    }
}
