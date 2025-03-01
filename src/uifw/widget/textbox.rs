// Copyright (C) 2025 Anders Sundman <anders@4zm.org>
//
// This file is part of RTRK - The Rust Tracker
//
// RTRK is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// RTRK is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with RTRK. If not, see <https://www.gnu.org/licenses/>.

use crate::uifw::interaction::{Event, Renderer, Style, CharModifiers};
use crate::uifw::pos::Pos;
use crate::uifw::widget::{Focusable, Task, View, Widget};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Message {
    EnterChar(char, CharModifiers),
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
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Widget<Message, (), TextBoxView> for TextBox {
    fn update(&mut self, msg: Message) -> Vec<Task<()>> {
        match msg {
            Message::EnterChar(c, _) => {
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

        vec![]
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
            Event::Char(c, m) => vec![Message::EnterChar(c,m)],
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
    use crate::uifw::interaction::tests::TestRenderer;

    #[test]
    fn textbox_test() {
        let txtbx = textbox(5);

        let btn_view = txtbx.view(Pos { r: 0, c: 0 });
        let mut renderer = TestRenderer::new();
        btn_view.draw(&mut renderer);
        assert_eq!(renderer.out, "-----");
    }
}
