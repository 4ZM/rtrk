use crate::interaction::{Event, Renderer, Style};
use crate::pos::Pos;
use crate::widget::{Focusable, View, Widget};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Message {
    EnterChar(char),
    Del,
    DelBack,
    CursorLeft,
    CursorRight,
}

pub struct TextBox {
    width: usize,
    carret_idx: usize,
    text: String,
    has_focus: bool,
}
impl TextBox {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            carret_idx: 0,
            text: " ".repeat(width).to_string(),
            has_focus: false,
        }
    }
    pub fn _text(&self) -> &str {
        &self.text
    }
}

impl Widget<Message, TextBoxView> for TextBox {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::EnterChar(c) => {
                self.text
                    .replace_range(self.carret_idx..self.carret_idx + 1, &c.to_string());

                if self.carret_idx < self.width - 1 {
                    self.carret_idx += 1;
                }
            }
            Message::Del => {
                self.text
                    .replace_range(self.carret_idx..self.carret_idx + 1, " ");
            }
            Message::DelBack => {
                if self.carret_idx > 0 {
                    self.carret_idx -= 1;
                }
                self.text
                    .replace_range(self.carret_idx..self.carret_idx + 1, " ");
            }
            Message::CursorRight => {
                self.carret_idx = (self.carret_idx + 1) % self.width;
            }
            Message::CursorLeft => {
                if self.carret_idx == 0 {
                    self.carret_idx = self.width - 1;
                } else {
                    self.carret_idx -= 1;
                }
            }
        }
    }
    fn view(&self, pos: Pos) -> TextBoxView {
        TextBoxView::new(pos, self.carret_idx, &self.text, self.has_focus)
    }
}
impl Focusable for TextBox {
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

pub struct TextBoxView {
    pos: Pos,
    text: String,
    has_focus: bool,
    carret_idx: usize,
}

impl TextBoxView {
    fn new(pos: Pos, carret_idx: usize, text: &str, has_focus: bool) -> Self {
        Self {
            pos,
            carret_idx,
            text: text.to_string().replace(" ", "-"),
            has_focus,
        }
    }
}
impl View<Message> for TextBoxView {
    fn on_event(&self, e: Event) -> Vec<Message> {
        if !self.has_focus {
            return vec![];
        }

        let msgs = match e {
            Event::Activate => vec![],
            Event::Char(c) => vec![Message::EnterChar(c)],
            Event::Left => vec![Message::CursorLeft],
            Event::Right => vec![Message::CursorRight],
            Event::Del => vec![Message::Del],
            Event::DelBack => vec![Message::DelBack],
            _ => vec![],
        };

        msgs
    }

    fn draw(&self, renderer: &mut dyn Renderer) {
        if self.has_focus {
            renderer.render_fmt_str(
                self.pos,
                format!("{}", &self.text[..self.carret_idx]).as_str(),
                Style::Highlight,
            );
            renderer.render_fmt_str(
                self.pos
                    + Pos {
                        r: 0,
                        c: self.carret_idx as u16,
                    },
                format!("{}", &self.text[self.carret_idx..self.carret_idx + 1]).as_str(),
                Style::Invert,
            );
            renderer.render_fmt_str(
                self.pos
                    + Pos {
                        r: 0,
                        c: self.carret_idx as u16 + 1,
                    },
                format!("{}", &self.text[self.carret_idx + 1..]).as_str(),
                Style::Highlight,
            );
        } else {
            renderer.render_str(self.pos, &self.text);
        }
    }
}

pub fn textbox(width: usize) -> TextBox {
    TextBox::new(width)
}

pub type TextBoxRc = Rc<RefCell<TextBox>>;
pub fn textbox_rc(width: usize) -> TextBoxRc {
    Rc::new(RefCell::new(textbox(width)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::{tests::TestRenderer, Event};

    #[test]
    fn textbox_test() {
        let txtbx = textbox(5);

        let btn_view = txtbx.view(Pos { r: 0, c: 0 });
        let mut renderer = TestRenderer::new();
        btn_view.draw(&mut renderer);
        assert_eq!(renderer.out, "-----");
    }
}
