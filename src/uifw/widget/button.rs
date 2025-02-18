use std::cell::RefCell;
use std::rc::Rc;

use crate::uifw::interaction::Event;
use crate::uifw::interaction::Renderer;
use crate::uifw::pos::Pos;
use crate::uifw::widget::{Focusable, View, Widget};

pub struct Button<Message> {
    text: String,
    on_press: Message,
    pub has_focus: bool,
}

impl<Message: Copy> Widget<Message, (), ButtonView<Message>> for Button<Message> {
    fn update(&mut self, _msg: Message) -> Vec<()> {
        vec![]
    }
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
    fn focus(&mut self) {
        self.has_focus = true
    }
    fn defocus(&mut self) {
        self.has_focus = false
    }
    fn next_focus(&mut self) {
        self.has_focus = !self.has_focus;
    }
    fn prev_focus(&mut self) {
        self.has_focus = !self.has_focus;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uifw::interaction::{tests::TestRenderer, Event};

    #[test]
    fn button_test() {
        let mut btn = button("BTN", 42);

        // Unless it's focused, it doesn't produce messages
        let btn_view = btn.view(Pos { r: 0, c: 0 });
        assert!(btn_view.on_event(Event::Activate).is_empty());

        let mut renderer = TestRenderer::new();
        btn_view.draw(&mut renderer);
        assert_eq!(renderer.out, " BTN ");

        // When focused, it can be activated
        btn.focus();
        let btn_view = btn.view(Pos { r: 0, c: 0 });
        let msg = btn_view.on_event(Event::Activate);
        assert_eq!(msg.len(), 1);
        assert_eq!(msg[0], 42);

        // Focused, it should also indicate that
        let mut renderer = TestRenderer::new();
        btn_view.draw(&mut renderer);
        assert_eq!(renderer.out, "[BTN]");
    }
}
