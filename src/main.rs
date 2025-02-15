mod interaction;
mod pos;
mod term;

// 1. DONE: Expand by adding reset-button in App
// challenging for focus logic currently
// 2. TODO: Expand by adding sum label in App
// challenging with root state that depends on component states
// 3. TODO: Expand by adding fraction of whole to spinner labels
// challenging since top App state is needed further down.

mod app {
    use crate::button::{button_rc, ButtonRc, ButtonView};
    use crate::impl_focusable_with_focuschain;
    use crate::interaction::Event;
    use crate::pos::Pos;
    use crate::spinner;
    use crate::spinner::{spinner_rc, SpinnerRc, SpinnerView};
    use crate::widget::{FocusChain, Focusable, FocusableRc, View, Widget};
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        Reset,
        NextFocus,
        Spinner(usize, spinner::Message),
    }

    pub struct App {
        pub spin: Vec<SpinnerRc>,
        focus_chain: FocusChain,
        pub reset_btn: ButtonRc<Message>,
    }
    impl App {
        pub fn new() -> Self {
            let spin = vec![spinner_rc(23), spinner_rc(42), spinner_rc(4711)];
            let reset_btn = button_rc("RST", Message::Reset);

            let mut focus_chain = FocusChain::new();
            for s in &spin {
                focus_chain.push(s.clone() as FocusableRc);
            }
            focus_chain.push(reset_btn.clone() as FocusableRc);

            Self {
                focus_chain,
                spin,
                reset_btn,
            }
        }
    }
    impl Widget<Message, AppView> for App {
        fn update(&mut self, msg: Message) {
            match msg {
                Message::Spinner(i, msg) => self.spin[i].borrow_mut().update(msg),
                Message::NextFocus => self.next_focus(),
                Message::Reset => self.spin.iter().for_each(|s| s.borrow_mut().value = 0),
            };
        }

        fn view(&self, pos: Pos) -> AppView {
            AppView {
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
        rst_btn: ButtonView<Message>,
        spinners: Vec<SpinnerView>,
    }
    impl View<Message> for AppView {
        fn draw(&self, renderer: &mut dyn crate::interaction::Renderer) {
            // TODO: Children can be used generically here
            for s in self.spinners.iter() {
                s.draw(renderer);
            }

            self.rst_btn.draw(renderer);
        }
        fn on_event(&self, e: Event) -> Vec<Message> {
            if let Event::Next = e {
                return vec![Message::NextFocus];
            }

            // TODO children() can be used generically here. Or can it? Spinner message is not generic
            // could have a ChildMessage routed by child ID or similar
            let mut msgs: Vec<Message> = vec![];
            for (i, s) in self.spinners.iter().enumerate() {
                let new_msgs = s.on_event(e);
                for m in new_msgs {
                    msgs.push(Message::Spinner(i, m));
                }
            }

            for &e in self.rst_btn.on_event(e).iter() {
                msgs.push(e);
            }

            msgs
        }
    }
}

mod spinner {
    use crate::button::{button_rc, ButtonRc, ButtonView};
    use crate::impl_focusable_with_focuschain;
    use crate::interaction::Event;
    use crate::label::{label, Label};
    use crate::pos::Pos;
    use crate::widget::{FocusChain, Focusable, FocusableRc, View, Widget};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        NextFocus,
        Increment,
        Decrement,
    }

    // TODO pub for test - ok?
    pub struct Spinner {
        pub value: i64,
        pub inc_btn: ButtonRc<Message>,
        pub dec_btn: ButtonRc<Message>,
        focus_chain: FocusChain,
    }
    impl Spinner {
        pub fn new(initial_value: i64) -> Self {
            let inc_btn = button_rc("+", Message::Increment);
            let dec_btn = button_rc("-", Message::Decrement);

            let mut focus_chain = FocusChain::new();
            focus_chain.push(inc_btn.clone() as FocusableRc);
            focus_chain.push(dec_btn.clone() as FocusableRc);

            Self {
                value: initial_value,
                inc_btn,
                dec_btn,
                focus_chain,
            }
        }
    }
    impl Widget<Message, SpinnerView> for Spinner {
        fn update(&mut self, msg: Message) {
            match msg {
                Message::Increment => self.value += 1,
                Message::Decrement => self.value -= 1,
                Message::NextFocus => self.next_focus(),
            }
        }

        fn view(&self, pos: Pos) -> SpinnerView {
            SpinnerView {
                lbl: label(pos + Pos { r: 1, c: 1 }, &self.value.to_string()),
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
                Event::Next => vec![Message::NextFocus],
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
    pub fn spinner_rc(value: i64) -> SpinnerRc {
        Rc::new(RefCell::new(Spinner::new(value)))
    }
}

mod button {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::interaction::Event;
    use crate::interaction::Renderer;
    use crate::pos::Pos;
    use crate::widget::{Focusable, View, Widget};

    pub struct Button<Message> {
        text: String,
        on_press: Message,
        pub has_focus: bool,
    }

    impl<Message: Copy> Widget<Message, ButtonView<Message>> for Button<Message> {
        fn update(&mut self, _msg: Message) {}
        fn view(&self, pos: Pos) -> ButtonView<Message> {
            ButtonView::<Message> {
                text: self.text.clone(),
                on_press: self.on_press,
                pos,
                has_focus: self.has_focus,
            }
        }
    }

    pub struct ButtonView<Message> {
        text: String,
        on_press: Message,
        pos: Pos,
        has_focus: bool,
    }

    impl<Message: Copy> View<Message> for ButtonView<Message> {
        fn on_event(&self, e: Event) -> Vec<Message> {
            match (e, self.has_focus) {
                (Event::Activate, true) => {
                    vec![self.on_press]
                }
                _ => vec![],
            }
        }
        fn draw(&self, renderer: &mut dyn Renderer) {
            if self.has_focus {
                renderer.render_str(self.pos, &format!("[{}]", &self.text));
            } else {
                renderer.render_str(self.pos, &format!(" {} ", &self.text));
            }
        }
    }
    impl<Message> Focusable for Button<Message> {
        fn has_focus(&self) -> bool {
            self.has_focus
        }
        fn next_focus(&mut self) {
            self.has_focus = !self.has_focus;
        }
        fn focus(&mut self) {
            self.has_focus = true
        }
        fn defocus(&mut self) {
            self.has_focus = false
        }
    }

    pub fn button<Message>(text: &str, on_press: Message) -> Button<Message> {
        Button {
            text: text.to_string(),
            on_press,
            has_focus: false,
        }
    }

    pub type ButtonRc<Message> = Rc<RefCell<Button<Message>>>;
    pub fn button_rc<Message>(text: &str, on_press: Message) -> ButtonRc<Message> {
        Rc::new(RefCell::new(button(text, on_press)))
    }
}

mod label {

    use crate::pos::Pos;
    use crate::widget::View;

    pub struct Label {
        pos: Pos,
        text: String,
    }

    impl View<()> for Label {
        fn draw(&self, renderer: &mut dyn crate::interaction::Renderer) {
            renderer.render_str(self.pos, &format!("{}", &self.text));
        }
    }
    pub fn label(pos: Pos, text: &str) -> Label {
        Label {
            pos,
            text: text.to_string(),
        }
    }
}

mod widget {
    // TODO : Design issue: Does it make sense to split widget and view?
    // If yes, how to reduce copy?
    // If no, what are the implications
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::interaction::{Event, Renderer};
    use crate::pos::Pos;

    #[macro_export]
    macro_rules! impl_focusable_with_focuschain {
        ($outer_type:ident, $inner_field:ident) => {
            impl Focusable for $outer_type {
                fn has_focus(&self) -> bool {
                    self.$inner_field.has_focus()
                }
                fn focus(&mut self) {
                    self.$inner_field.focus();
                }
                fn defocus(&mut self) {
                    self.$inner_field.defocus();
                }
                fn next_focus(&mut self) {
                    self.$inner_field.next_focus();
                }
            }
        };
    }

    pub struct FocusChain {
        pub focus_idx: Option<usize>,
        pub focusables: Vec<FocusableRc>,
    }

    impl Focusable for FocusChain {
        fn has_focus(&self) -> bool {
            self.focus_idx.is_some()
        }

        fn defocus(&mut self) {
            for f in self.focusables.iter_mut() {
                f.borrow_mut().defocus();
            }
            self.focus_idx = None;
        }
        fn focus(&mut self) {
            // Reset to get first widget in tree
            self.focus_idx = None;
            self.next_focus();
        }

        fn next_focus(&mut self) {
            self.focus_idx = match self.focus_idx {
                None => {
                    // Start a new focus cycle
                    self.focusables[0].borrow_mut().next_focus();
                    Some(0)
                }
                Some(idx) => {
                    // Advance the child tree
                    self.focusables[idx].borrow_mut().next_focus();

                    // Still same child tree that has focus?
                    if self.focusables[idx].borrow().has_focus() {
                        Some(idx)
                    } else {
                        // Child tree lost focus
                        if idx == self.focusables.len() - 1 {
                            // Last subtree lost focus, nothing left
                            None
                        } else {
                            // Start traversing next subtree
                            self.focusables[idx + 1].borrow_mut().next_focus();
                            Some(idx + 1)
                        }
                    }
                }
            };
        }
    }

    impl FocusChain {
        pub fn new() -> Self {
            Self {
                focus_idx: None,
                focusables: vec![],
            }
        }

        pub fn push(&mut self, focusable: Rc<RefCell<dyn Focusable>>) {
            self.focusables.push(focusable);
        }
    }

    pub type FocusableRc = Rc<RefCell<dyn Focusable>>;
    pub trait Focusable {
        /// Has focus directly or if any of it's children has focus
        fn has_focus(&self) -> bool;
        fn focus(&mut self);
        fn defocus(&mut self);

        /// Advance focus recursively
        fn next_focus(&mut self);
    }

    /// A Widget is statefull and has the update() mechanism to mutate it's state.
    /// It create views that are entirely disconnected from itself (no back ref with a lifetime).
    pub trait Widget<Message, V: View<Message>>: Focusable {
        fn update(&mut self, msg: Message);
        fn view(&self, pos: Pos) -> V;
    }

    /// A View can have data, but is stateless and immutable.  It's purpose is to interact with the
    /// UI framework wrappers to draw and translate events to application messages.  Some things are
    /// "just views" like labels (no state), some things like buttons are almost no state but are
    /// focusable (i.e. have state)
    pub trait View<Message> {
        fn on_event(&self, _e: Event) -> Vec<Message> {
            vec![]
        }
        fn draw(&self, _renderer: &mut dyn Renderer);
    }
}

mod runtime {

    use crate::app;
    use crate::interaction;
    use crate::interaction::EventCollector;
    use crate::interaction::Renderer;
    use crate::pos::Pos;
    use crate::term;
    use crate::widget::View;
    use crate::widget::Widget;
    use std::time::Duration;

    // Runtime
    pub fn start() {
        let mut renderer = term::CrosstermRenderer::new(std::io::stdout());
        let event_collector = term::CrosstermEventCollector {};

        let mut app = app::App::new();
        //        app.inc_btn.focus();

        'app: loop {
            // Render state
            let view = app.view(Pos { r: 0, c: 0 });
            renderer.clear();
            view.draw(&mut renderer);
            renderer.flush();

            std::thread::sleep(Duration::from_millis(20));

            // Get UI event interactions
            let mut unprocessed_messages: Vec<app::Message> = vec![];
            for event in event_collector.poll_events() {
                let mut event_messages = match event {
                    interaction::Event::Quit => break 'app,
                    _ => view.on_event(event),
                };

                unprocessed_messages.append(&mut event_messages);
            }

            // Update widgets
            while let Some(msg) = unprocessed_messages.pop() {
                app.update(msg);
            }
        }
    }
}

fn main() {
    runtime::start();
}

#[cfg(test)]
mod tests {

    use interaction::Renderer;

    use super::button::*;
    use super::label::*;
    use super::pos::*;
    use super::widget::*;
    use super::*;

    struct TestRenderer {
        pub out: String,
    }
    impl TestRenderer {
        fn new() -> Self {
            Self { out: String::new() }
        }
    }
    impl Renderer for TestRenderer {
        fn clear(&mut self) {}
        fn flush(&mut self) {}
        fn render_str(&mut self, _pos: Pos, text: &str) {
            self.out += text;
        }
    }

    #[test]
    fn spinner_state_update_test() {
        let mut app = spinner::Spinner::new(0);
        assert_eq!(app.value, 0);

        let _ = app.update(spinner::Message::Increment);
        app.update(spinner::Message::Increment);
        app.update(spinner::Message::Decrement);

        assert_eq!(app.value, 1);
    }

    #[test]
    fn spinner_rendering_test() {
        let app = spinner::Spinner::new(0);
        assert_eq!(app.value, 0);

        let view = app.view(Pos { r: 0, c: 0 });
        let mut renderer = TestRenderer::new();
        view.draw(&mut renderer);
        assert_eq!(renderer.out, " + 0 - ");

        app.inc_btn.borrow_mut().focus();
        let view = app.view(Pos { r: 0, c: 0 });
        let mut renderer = TestRenderer::new();
        view.draw(&mut renderer);
        assert_eq!(renderer.out, "[+]0 - ");
    }

    #[test]
    fn button_test() {
        let mut btn = button("BTN", 42);

        // Unless it's focused, it doesn't produce messages
        let btn_view = btn.view(Pos { r: 0, c: 0 });
        assert!(btn_view.on_event(interaction::Event::Activate).is_empty());

        let mut renderer = TestRenderer::new();
        btn_view.draw(&mut renderer);
        assert_eq!(renderer.out, " BTN ");

        // When focused, it can be activated
        btn.focus();
        let btn_view = btn.view(Pos { r: 0, c: 0 });
        let msg = btn_view.on_event(interaction::Event::Activate);
        assert_eq!(msg.len(), 1);
        assert_eq!(msg[0], 42);

        // Focused, it should also indicate that
        let mut renderer = TestRenderer::new();
        btn_view.draw(&mut renderer);
        assert_eq!(renderer.out, "[BTN]");
    }

    #[test]
    fn label_test() {
        let lbl = label(Pos { r: 0, c: 0 }, "LBL");

        let mut renderer = TestRenderer::new();
        lbl.draw(&mut renderer);
        assert_eq!(renderer.out, "LBL");

        // Can't activate a label
        let msg = lbl.on_event(interaction::Event::Activate);
        assert!(msg.is_empty());
    }

    #[test]
    fn focus_helper_test() {
        fn focus_helper(mut f: Vec<&mut dyn Focusable>) {
            f[0].focus();
        }

        let mut b1 = button("BTN", 0);
        let mut b2 = button("BTN", 0);

        focus_helper(vec![&mut b1, &mut b2]);
    }
}
