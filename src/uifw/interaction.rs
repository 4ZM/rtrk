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

/// The abstract interraction interface between the Application and the UI framework.
/// Different UI frameworks can implement this (e.g. terminal, web, etc).
use crate::uifw::pos::Pos;
use bitflags::bitflags;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CharModifiers(u8);
bitflags! {
    impl CharModifiers: u8 {
        const None = 0b0000_0000;
        const Shift = 0b0000_0001;
        const Ctrl = 0b0000_0010;
        const Alt = 0b0000_0100;
        const CapsLock = 0b0000_1000;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Event {
    NextFocus,
    PrevFocus,
    Activate,
    Quit,
    Char(char, CharModifiers),
    Del,
    DelBack,
    Right,
    Left,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Style {
    Default,
    Invert,
    Highlight,
}

pub trait Renderer {
    fn clear(&mut self);
    fn render_str(&mut self, _pos: Pos, _text: &str);
    fn render_fmt_str(&mut self, _pos: Pos, _text: &str, _fmt: Style);
    fn flush(&mut self);
}

pub trait EventCollector {
    fn poll_events(&self) -> Vec<Event>;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub struct TestRenderer {
        pub out: String,
    }
    impl TestRenderer {
        pub fn new() -> Self {
            Self { out: String::new() }
        }
    }
    impl Renderer for TestRenderer {
        fn clear(&mut self) {}
        fn flush(&mut self) {}
        fn render_str(&mut self, _pos: Pos, text: &str) {
            self.out += text;
        }
        fn render_fmt_str(&mut self, _pos: Pos, text: &str, _style: Style) {
            self.out += text;
        }
    }
}
