mod interaction;
mod pos;
mod term;

// 1. TODO: Expand by adding reset-button in App
// challenging for focus logic currently
// 2. TODO: Expand by adding sum label in App
// challenging with root state that depends on component states
// 3. TODO: Expand by adding fraction of whole to spinner labels
// challenging since top App state is needed further down.

mod app {
    use crate::interaction::Event;
    use crate::pos::Pos;
    use crate::spinner;
    use crate::spinner::{Spinner, SpinnerView};
    use crate::widget::{Focusable, View, Widget};

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        NextFocus,
        Spinner(usize, spinner::Message),
    }

    pub struct App {
        pub spin: Vec<Spinner>,
        pub focus: Option<usize>,
    }
    impl App {
        pub fn new() -> Self {
            Self {
                focus: None,
                spin: vec![Spinner::new(23), Spinner::new(42), Spinner::new(4711)],
            }
        }
    }
    impl Widget<Message, AppView> for App {
        fn update(&mut self, msg: Message) {
            match msg {
                Message::Spinner(i, msg) => self.spin[i].update(msg),
                Message::NextFocus => self.next_focus(),
            };
        }

        fn view(&self, pos: Pos) -> AppView {
            AppView {
                spinners: self
                    .spin
                    .iter()
                    .enumerate()
                    .map(|(i, s)| {
                        s.view(
                            pos + Pos {
                                r: 0,
                                c: i as u16 * 10, // Horizontal spaced
                            },
                        )
                    })
                    .collect(),
            }
        }
    }
    impl Focusable for App {
        fn has_focus(&self) -> bool {
            self.focus.is_some()
        }

        fn defocus(&mut self) {
            // TODO: children() can be used here
            for s in self.spin.iter_mut() {
                s.defocus();
            }
            self.focus = None;
        }
        fn focus(&mut self) {
            self.defocus();
            self.update(Message::NextFocus);
        }

        fn next_focus(&mut self) {
            // TODO: children() can be used here for generic tree traversal

            self.focus = match self.focus {
                None => {
                    // Start a new focus cycle
                    self.spin[0].next_focus();
                    Some(0)
                }
                Some(idx) => {
                    // Advance the child tree
                    self.spin[idx].next_focus();

                    // Still same child tree that has focus?
                    if self.spin[idx].has_focus() {
                        Some(idx)
                    } else {
                        // Child tree lost focus
                        if idx == self.spin.len() - 1 {
                            // Last subtree lost focus, nothing left
                            None
                        } else {
                            // Start traversing next subtree
                            self.spin[idx + 1].next_focus();
                            Some(idx + 1)
                        }
                    }
                }
            };
        }
    }

    pub struct AppView {
        spinners: Vec<SpinnerView>,
    }
    impl View<Message> for AppView {
        fn draw(&self, renderer: &mut dyn crate::interaction::Renderer) {
            // TODO: Children can be used generically here
            for s in self.spinners.iter() {
                s.draw(renderer);
            }
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

            msgs
        }
    }
}

mod spinner {
    use crate::button::{button, Button, ButtonView};
    use crate::interaction::Event;
    use crate::label::{label, Label};
    use crate::pos::Pos;
    use crate::widget::{Focusable, View, Widget};

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        NextFocus,
        Increment,
        Decrement,
    }

    pub struct Spinner {
        pub value: i64,               // TODO pub for test - ok?
        pub inc_btn: Button<Message>, // they have the focus state, so can't be just view
        pub dec_btn: Button<Message>,
    }
    impl Spinner {
        pub fn new(initial_value: i64) -> Self {
            Self {
                value: initial_value,
                inc_btn: button("+", Message::Increment),
                dec_btn: button("-", Message::Decrement),
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
                inc_btn: self.inc_btn.view(pos + Pos { r: 0, c: 0 }),
                dec_btn: self.dec_btn.view(pos + Pos { r: 2, c: 0 }),
            }
        }
    }
    impl Focusable for Spinner {
        fn has_focus(&self) -> bool {
            self.inc_btn.has_focus() || self.dec_btn.has_focus()
        }

        fn next_focus(&mut self) {
            if self.inc_btn.has_focus() {
                self.inc_btn.defocus();
                self.dec_btn.focus();
            } else if self.dec_btn.has_focus() {
                self.dec_btn.defocus();
            } else {
                self.inc_btn.focus();
            }
        }
        fn defocus(&mut self) {
            self.inc_btn.defocus();
            self.dec_btn.defocus();
        }
        fn focus(&mut self) {
            self.inc_btn.focus();
            self.dec_btn.defocus();
        }
    }

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
}

mod button {
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

    // TODO Reduce verbosity of Message: Copy constraint?
    pub fn button<Message>(text: &str, on_press: Message) -> Button<Message> {
        Button {
            text: text.to_string(),
            on_press,
            has_focus: false,
        }
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

    use crate::interaction::{Event, Renderer};
    use crate::pos::Pos;

    // TODO: Focusable as trait only make sense in polymorphic widget usecase. Remove it?
    pub trait Focusable {
        /// Has focus directly or if any of it's children has focus
        fn has_focus(&self) -> bool;
        fn focus(&mut self);
        fn defocus(&mut self);

        /// Advance focus recursively
        fn next_focus(&mut self);
    }

    // TODO : Can we get rid of the noisy Copy constrait here?
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
                //let mut new_messages =
                app.update(msg);
                //unprocessed_messages.append(&mut new_messages);
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
        let mut app = spinner::Spinner::new(0);
        assert_eq!(app.value, 0);

        let view = app.view(Pos { r: 0, c: 0 });
        let mut renderer = TestRenderer::new();
        view.draw(&mut renderer);
        assert_eq!(renderer.out, " + 0 - ");

        app.inc_btn.focus();
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
