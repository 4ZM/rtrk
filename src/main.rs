mod pos;

mod app {
    use crate::button::{button, Button, ButtonView};
    use crate::label::{label, Label};
    use crate::pos::Pos;
    use crate::widget::{Focusable, View, Widget};

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Message {
        Increment,
        Decrement,
    }

    pub struct App {
        pos: Pos,
        pub value: i64, // TODO pub for test - ok?
        pub inc_btn: Button<Message>,
        pub dec_btn: Button<Message>,
        //        pub lbl: Label,
    }
    impl App {
        pub fn new() -> Self {
            let pos = Pos::default();
            Self {
                pos,
                value: 0,
                inc_btn: button(pos + Pos { r: 0, c: 0 }, "+", Message::Increment),
                dec_btn: button(pos + Pos { r: 2, c: 0 }, "-", Message::Decrement),
                //              lbl: label(pos + Pos { r: 0, c: 3 }, ""),
            }
        }
    }
    impl<'a> Widget<'a, Message, AppView<'a>> for App {
        fn pos(&self) -> Pos {
            self.pos
        }

        fn update(&mut self, msg: Message) -> Vec<Message> {
            match msg {
                Message::Increment => self.value += 1,
                Message::Decrement => self.value -= 1,
            }
            vec![]
        }

        fn view(&'a self) -> AppView {
            AppView {
                widget: &self,
                lbl: label(self.pos + Pos { r: 1, c: 1 }, &self.value.to_string()),
            }
            // Layout {
            //     views: vec![
            //         Box::new(ButtonView::<'a, Message> {
            //             button: &self.inc_btn,
            //         }),
            //         Box::new(ButtonView::<'a, Message> {
            //             button: &self.dec_btn,
            //         }),
            //     ],
            //     //                views: vec![],
            // }
        }
    }
    impl Focusable for App {}
    impl<Message: Copy> View<Message> for App {
        fn draw(&self, _renderer: &mut dyn crate::interaction::Renderer) {}
    }

    pub struct AppView<'a> {
        widget: &'a App,
        lbl: Label,
    }
    impl<'a> View<Message> for AppView<'a> {
        fn draw(&self, renderer: &mut dyn crate::interaction::Renderer) {
            self.widget.inc_btn.view().draw(renderer);
            self.lbl.draw(renderer);
            self.widget.dec_btn.view().draw(renderer);
        }
        fn on_event(&self, e: crate::interaction::Event) -> Vec<Message> {
            vec![
                self.widget.inc_btn.view().on_event(e),
                self.widget.dec_btn.view().on_event(e),
            ]
            .concat()
        }
    }
}
mod layout {
    // use crate::interaction::Renderer;
    // use crate::widget::View;

    // pub struct Layout<Message> {
    //     pub views: Vec<Box<dyn View<Message>>>,
    // }
    // impl<Message> View<Message> for Layout<Message> {
    //     fn on_event(&self, e: crate::interaction::Event) -> Vec<Message> {
    //         let mut messages: Vec<Message> = vec![];
    //         for v in self.views.iter() {
    //             let mut new_messages = v.as_ref().on_event(e);
    //             messages.append(&mut new_messages);
    //         }
    //         messages
    //     }

    //     fn draw(&self, renderer: &mut dyn Renderer) {
    //         for v in self.views.iter() {
    //             v.as_ref().draw(renderer);
    //         }
    //     }
    // }
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
        has_focus: bool,
    }
    impl<'a, Message: Copy> Widget<'a, Message, ButtonView<'a, Message>> for Button<Message> {
        fn pos(&self) -> Pos {
            self.pos
        }
        fn update(&mut self, _msg: Message) -> Vec<Message> {
            vec![]
        }
        fn view(&'a self) -> ButtonView<'a, Message> {
            ButtonView::<Message> { button: &self }
        }
    }

    pub struct ButtonView<'a, Message: Copy> {
        pub button: &'a Button<Message>, // TODO shouldn't be pub, create a ctor
    }

    impl<'a, Message: Copy> View<Message> for ButtonView<'a, Message> {
        fn on_event(&self, e: Event) -> Vec<Message> {
            match (e, self.button.has_focus) {
                (Event::Activate, true) => {
                    vec![self.button.on_press]
                }
                _ => vec![],
            }
        }
        fn draw(&self, renderer: &mut dyn Renderer) {
            if self.button.has_focus {
                renderer.render_str(self.button.pos, &format!("[{}]", &self.button.text));
            } else {
                renderer.render_str(self.button.pos, &format!(" {} ", &self.button.text));
            }
        }
    }
    impl<Message: Copy> Focusable for Button<Message> {
        fn accepts_focus(&self) -> bool {
            true
        }
        fn focus(&mut self) {
            self.has_focus = true
        }
        fn defocus(&mut self) {
            self.has_focus = false
        }
    }

    // TODO Reduce verbosity of Message: Copy constraint?

    pub fn button<Message: Copy>(pos: Pos, text: &str, on_press: Message) -> Button<Message> {
        Button::<Message> {
            pos,
            text: text.to_string(),
            on_press,
            has_focus: false,
        }
    }
}

mod label {

    use crate::pos::Pos;
    use crate::widget::{Focusable, View, Widget};

    pub struct Label {
        pos: Pos,
        text: String,
    }

    // impl<'a> Widget<'a, (), LabelView<'a>> for Label {
    //     fn pos(&self) -> Pos {
    //         self.pos
    //     }
    //     fn update(&mut self, _msg: ()) -> Vec<()> {
    //         vec![]
    //     }
    //     fn view(&'a self) -> LabelView<'a> {
    //         LabelView { label: &self }
    //     }
    // }

    // pub struct LabelView<'a> {
    //     pub label: &'a Label,
    // }

    impl View<()> for Label {
        fn draw(&self, renderer: &mut dyn crate::interaction::Renderer) {
            renderer.render_str(self.pos, &format!("{}", &self.text));
        }
    }
    //impl Focusable for Label {}
    pub fn label(pos: Pos, text: &str) -> Label {
        Label {
            pos,
            text: text.to_string(),
        }
    }
}

// mod label {
//     use crate::interaction::Renderer;
//     use crate::pos::Pos;
//     use crate::widget::{Focusable, View, Widget};

//     impl<Message, V: View> Widget<Message, V> for Label {
//         fn pos(&self) -> Pos {
//             self.pos
//         }
//         fn update(&mut self, msg: Message) -> Vec<Message> {
//             vec![]
//         }
//         fn view(&self) -> V {
//             self
//         }
//     }

//     impl View for Label {
//         fn draw(&self, renderer: &mut dyn Renderer) {
//             renderer.render_str(self.pos, &self.text);
//         }
//     }
//     impl Focusable for Label {}
//     pub fn label(pos: Pos, text: &str) -> Box<Label> {
//         Box::new(Label {
//             pos,
//             text: text.to_string(),
//         })
//     }
// }

mod widget {
    use crate::interaction::{Event, Renderer};
    use crate::pos::Pos;

    pub trait Focusable {
        fn accepts_focus(&self) -> bool {
            false
        }
        fn focus(&mut self) {}
        fn defocus(&mut self) {}
    }

    pub trait Widget<'a, Message: Copy, V: View<Message>>: Focusable {
        fn pos(&self) -> Pos;

        fn update(&mut self, msg: Message) -> Vec<Message>; // TODO return message should be different type

        fn view(&'a self) -> V;
    }

    pub trait View<Message> {
        fn on_event(&self, e: Event) -> Vec<Message> {
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
        Back,
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
    use crate::widget::{Focusable, Widget};
    use std::time::Duration;

    // Runtime
    pub fn start() {
        let mut renderer = term::CrosstermRenderer::new(std::io::stdout());
        let event_collector = term::CrosstermEventCollector {};

        let mut app = app::App::new();
        app.inc_btn.focus();

        'app: loop {
            // Render state
            let view = app.view();
            view.draw(&mut renderer);
            renderer.flush();

            std::thread::sleep(Duration::from_millis(200));

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
                let mut new_messages = app.update(msg);
                unprocessed_messages.append(&mut new_messages);
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
        assert_eq!(renderer.out, " +  - ");

        app.inc_btn.focus();
        let view = app.view();
        let mut renderer = TestRenderer::new();
        view.draw(&mut renderer);
        assert_eq!(renderer.out, "[+] - ");
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

        // Unless it's focused, it doesn't produce messages
        let lbl_view = lbl.view();

        let mut renderer = TestRenderer::new();
        lbl_view.draw(&mut renderer);
        assert_eq!(renderer.out, "LBL");

        // Can't focus a label
        assert!(!lbl.accepts_focus());

        // Can't activate a label
        let msg = lbl_view.on_event(interaction::Event::Activate);
        assert!(msg.is_empty());

        // Nothing to do for label update
        let msg = lbl.update(());
        assert!(msg.is_empty());
    }
}
