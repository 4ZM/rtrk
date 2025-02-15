mod pos;

mod app {
    use crate::interaction::Event;
    use crate::pos::Pos;
    use crate::spinner;
    use crate::spinner::{Spinner, SpinnerView};
    use crate::widget::{Focusable, View, Widget};
    use itertools::Itertools;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        NextFocus,
        Spinner(usize, spinner::Message),
    }

    pub struct App {
        pub spin: Vec<Spinner>,
    }
    impl App {
        pub fn new() -> Self {
            Self {
                spin: vec![
                    Spinner::new(Pos { r: 0, c: 0 }, 23),
                    Spinner::new(Pos { r: 0, c: 10 }, 42),
                    Spinner::new(Pos { r: 0, c: 20 }, 4711),
                ],
            }
        }
    }
    impl Widget<Message, AppView> for App {
        // fn pos(&self) -> Pos {
        //     self.pos
        // }

        fn update(&mut self, msg: Message) {
            match msg {
                Message::Spinner(i, msg) => self.spin[i].update(msg),
                Message::NextFocus => {
                    let spinners = self.spin.len();

                    match self.spin.iter().find_position(|&s| s.has_focus()) {
                        Some((focused_spinner_idx, _)) => {
                            self.spin[focused_spinner_idx].update(spinner::Message::NextFocus);
                            if !self.spin[focused_spinner_idx].has_focus() {
                                // Moving on to next spinner unless it's the last
                                if focused_spinner_idx != spinners - 1 {
                                    self.spin[(focused_spinner_idx + 1usize) % spinners]
                                        .update(spinner::Message::NextFocus);
                                }
                            }
                        }
                        None => {
                            self.spin[0].update(spinner::Message::NextFocus);
                        }
                    }
                }
            };
        }

        fn view(&self) -> AppView {
            AppView {
                spinners: self.spin.iter().map(|s| s.view()).collect(),
            }
        }
    }
    impl Focusable for App {
        // fn has_focus(&self) -> bool {
        //     true
        // }
        // fn accepts_focus(&self) -> bool {
        //     true
        // }
        // fn defocus(&mut self) {}
        // fn focus(&mut self) {}
    }
    // impl<Message: Copy> View<Message> for App {
    //     fn draw(&self, _renderer: &mut dyn crate::interaction::Renderer) {}
    // }

    pub struct AppView {
        spinners: Vec<SpinnerView>,
    }
    impl View<Message> for AppView {
        fn draw(&self, renderer: &mut dyn crate::interaction::Renderer) {
            for s in self.spinners.iter() {
                s.draw(renderer);
            }
        }
        fn on_event(&self, e: Event) -> Vec<Message> {
            if let Event::Next = e {
                return vec![Message::NextFocus];
            }

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
        pos: Pos,
        pub value: i64,               // TODO pub for test - ok?
        pub inc_btn: Button<Message>, // they have the focus state, so can't be just view
        pub dec_btn: Button<Message>,
    }
    impl Spinner {
        pub fn new(pos: Pos, initial_value: i64) -> Self {
            Self {
                pos,
                value: initial_value,
                inc_btn: button(pos + Pos { r: 0, c: 0 }, "+", Message::Increment),
                dec_btn: button(pos + Pos { r: 2, c: 0 }, "-", Message::Decrement),
            }
        }
    }
    impl Widget<Message, SpinnerView> for Spinner {
        // fn pos(&self) -> Pos {
        //     self.pos
        // }

        fn update(&mut self, msg: Message) {
            match msg {
                Message::Increment => self.value += 1,
                Message::Decrement => self.value -= 1,
                Message::NextFocus if self.inc_btn.has_focus() => {
                    self.inc_btn.defocus();
                    self.dec_btn.focus();
                }
                Message::NextFocus if self.dec_btn.has_focus() => {
                    self.dec_btn.defocus();
                }
                Message::NextFocus => {
                    self.inc_btn.focus();
                }
            }
        }

        fn view(&self) -> SpinnerView {
            SpinnerView {
                lbl: label(self.pos + Pos { r: 1, c: 1 }, &self.value.to_string()),
                inc_btn: self.inc_btn.view(),
                dec_btn: self.dec_btn.view(),
            }
        }
    }
    impl Focusable for Spinner {
        fn has_focus(&self) -> bool {
            self.inc_btn.has_focus() || self.dec_btn.has_focus()
        }

        // TODO DO we really need it? Only makes sense in polymorphic case...
        // fn accepts_focus(&self) -> bool {
        //     true
        // }

        fn defocus(&mut self) {
            self.inc_btn.defocus();
            self.dec_btn.defocus();
        }
        fn focus(&mut self) {
            self.inc_btn.focus(); // Randomly pick one to focus -- THIS IS WRONG
        }
    }
    // impl<Message: Copy> View<Message> for Spinner {
    //     fn draw(&self, _renderer: &mut dyn crate::interaction::Renderer) {}
    // }

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

    pub struct Button<Message: Copy> {
        text: String,
        on_press: Message,
        pos: Pos,
        pub has_focus: bool,
    }
    impl<Message: Copy> Widget<Message, ButtonView<Message>> for Button<Message> {
        // fn pos(&self) -> Pos {
        //     self.pos
        // }
        fn update(&mut self, _msg: Message) {}
        fn view(&self) -> ButtonView<Message> {
            ButtonView::<Message> {
                text: self.text.clone(),
                on_press: self.on_press,
                pos: self.pos,
                has_focus: self.has_focus,
            }
        }
    }

    pub struct ButtonView<Message: Copy> {
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
    impl<Message: Copy> Focusable for Button<Message> {
        fn has_focus(&self) -> bool {
            self.has_focus
        }
        // fn accepts_focus(&self) -> bool {
        //     true
        // }
        fn focus(&mut self) {
            self.has_focus = true
        }
        fn defocus(&mut self) {
            self.has_focus = false
        }
    }

    // TODO Reduce verbosity of Message: Copy constraint?

    pub fn button<Message: Copy>(pos: Pos, text: &str, on_press: Message) -> Button<Message> {
        Button {
            pos,
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
    use crate::interaction::{Event, Renderer};
    //use crate::pos::Pos;

    pub trait Focusable {
        fn has_focus(&self) -> bool {
            false
        }
        // fn accepts_focus(&self) -> bool {
        //     false
        // }
        fn focus(&mut self) {}
        fn defocus(&mut self) {}
    }

    /// A Widget is statefull and has the update() mechanism to mutate it's state.
    /// It create views that are entirely disconnected from itself (no back ref with a lifetime).
    pub trait Widget<Message: Copy, V: View<Message>>: Focusable {
        //fn pos(&self) -> Pos;

        fn update(&mut self, msg: Message);

        fn view(&self) -> V;
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

mod interaction {
    use crate::pos::Pos;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Event {
        Next,
        //Back, // TODO Add back support
        Activate,
        Quit,
        Char(char),
    }

    pub trait Renderer {
        fn clear(&mut self);
        fn render_str(&mut self, _pos: Pos, _text: &str);
        fn flush(&mut self);
    }

    pub trait EventCollector {
        fn poll_events(&self) -> Vec<crate::interaction::Event>;
    }
}

mod term {
    // Crossterm adapter for the interactions
    use crate::interaction::EventCollector;
    use crate::interaction::Renderer;
    use crate::pos::Pos;
    use crossterm::event::KeyEventKind;
    use std::io;
    use std::time::Duration;

    pub use crossterm::{
        cursor,
        event::{self, KeyCode, KeyEvent},
        execute, queue, style,
        terminal::{self, ClearType},
    };

    pub struct CrosstermRenderer<W: io::Write> {
        w: W,
    }
    impl<W: io::Write> CrosstermRenderer<W> {
        pub fn new(mut w: W) -> CrosstermRenderer<W> {
            //let mut stdout = io::stdout();
            execute!(w.by_ref(), terminal::EnterAlternateScreen)
                .expect("Unable to create CrosstermRenderer");
            terminal::enable_raw_mode().expect("Unable to create CrosstermRenderer");

            CrosstermRenderer { w }
        }
    }
    impl<W: io::Write> Drop for CrosstermRenderer<W> {
        fn drop(&mut self) {
            execute!(
                self.w,
                style::ResetColor,
                cursor::Show,
                terminal::LeaveAlternateScreen
            )
            .expect("Unable to drop CrosstermRenderer");

            terminal::disable_raw_mode().expect("Unable to drop CrosstermRenderer");
        }
    }
    impl<W: io::Write> Renderer for CrosstermRenderer<W> {
        fn clear(&mut self) {
            queue!(self.w, terminal::Clear(ClearType::All),).expect("Unable to clear terminal");
        }
        fn flush(&mut self) {
            self.w.flush().expect("Unable to flush writer");
        }

        fn render_str(&mut self, Pos { r, c }: Pos, text: &str) {
            let _ = queue!(
                self.w,
                cursor::Hide,
                cursor::MoveTo(c, r),
                style::Print(format!("{}", text))
            );
        }
    }

    pub struct CrosstermEventCollector {}
    impl EventCollector for CrosstermEventCollector {
        fn poll_events(&self) -> Vec<crate::interaction::Event> {
            if !event::poll(Duration::from_secs(0)).unwrap() {
                // TODO handle error
                return vec![];
            }

            match event::read() {
                Ok(crossterm::event::Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                })) => return vec![crate::interaction::Event::Quit],
                Ok(crossterm::event::Event::Key(KeyEvent {
                    code: KeyCode::Tab,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                })) => return vec![crate::interaction::Event::Next],
                Ok(crossterm::event::Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                })) => return vec![crate::interaction::Event::Activate],
                Ok(crossterm::event::Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                })) => return vec![crate::interaction::Event::Char(c)],
                _ => vec![],
            }
        }
    }
}

mod runtime {

    use crate::app;
    use crate::interaction;
    use crate::interaction::EventCollector;
    use crate::interaction::Renderer;
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
            let view = app.view();
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
    fn basic_state_update_test() {
        let mut app = app::App::new();
        assert_eq!(app.value, 0);

        let _ = app.update(app::Message::Increment);
        app.update(app::Message::Increment);
        app.update(app::Message::Decrement);

        assert_eq!(app.value, 1);
    }

    #[test]
    fn spinner_test() {
        let mut app = app::App::new();
        assert_eq!(app.value, 0);

        let view = app.view();
        let mut renderer = TestRenderer::new();
        view.draw(&mut renderer);
        assert_eq!(renderer.out, " + 0 - ");

        app.inc_btn.focus();
        let view = app.view();
        let mut renderer = TestRenderer::new();
        view.draw(&mut renderer);
        assert_eq!(renderer.out, "[+]0 - ");
    }

    #[test]
    fn button_test() {
        let mut btn = button(Pos { r: 0, c: 0 }, "BTN", 42);

        // Unless it's focused, it doesn't produce messages
        let btn_view = btn.view();
        assert!(btn_view.on_event(interaction::Event::Activate).is_empty());

        let mut renderer = TestRenderer::new();
        btn_view.draw(&mut renderer);
        assert_eq!(renderer.out, " BTN ");

        // When focused, it can be activated
        btn.focus();
        let btn_view = btn.view();
        let msg = btn_view.on_event(interaction::Event::Activate);
        assert_eq!(msg.len(), 1);
        assert_eq!(msg[0], 42);

        // Focused, it should also indicate that
        let mut renderer = TestRenderer::new();
        btn_view.draw(&mut renderer);
        assert_eq!(renderer.out, "[BTN]");

        // Nothing to do for button update
        let msg = btn.update(42);
        assert!(msg.is_empty());
    }

    #[test]
    fn label_test() {
        let mut lbl = label(Pos { r: 0, c: 0 }, "LBL");

        let mut renderer = TestRenderer::new();
        lbl.draw(&mut renderer);
        assert_eq!(renderer.out, "LBL");

        // Can't activate a label
        let msg = lbl.on_event(interaction::Event::Activate);
        assert!(msg.is_empty());
    }
}
