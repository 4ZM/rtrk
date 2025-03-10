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

// Crossterm adapter for the interactions
use crate::uifw::interaction::Renderer;
use crate::uifw::interaction::Style;
use crate::uifw::interaction::{CharModifiers, EventCollector};
use crate::uifw::pos::Pos;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyEventState;
use crossterm::style::ResetColor;
use std::io;
use std::time::Duration;

const MIN_TERM_WIDTH: u16 = 80;
const MIN_TERM_HEIGHT: u16 = 24;

pub use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute, queue, style,
    style::{Attribute, Color, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
};

pub struct CrosstermRenderer<W: io::Write> {
    term_size: Pos,
    w: W,
}
impl<W: io::Write> CrosstermRenderer<W> {
    pub fn new(mut w: W) -> CrosstermRenderer<W> {
        terminal::enable_raw_mode().expect("Unable to create CrosstermRenderer");
        execute!(w.by_ref(), terminal::EnterAlternateScreen,)
            .expect("Unable to create CrosstermRenderer");

        let (c, r) = terminal::size().expect("Unable to determine size of terminal");

        CrosstermRenderer {
            w,
            term_size: Pos { r, c },
        }
    }

    fn ui_offset(&self, Pos { r, c }: Pos) -> Option<Pos> {
        if self.term_size.r < MIN_TERM_HEIGHT || self.term_size.c < MIN_TERM_WIDTH {
            return None;
        }
        Some(Pos {
            r: (self.term_size.r - MIN_TERM_HEIGHT) / 2 + r,
            c: (self.term_size.c - MIN_TERM_WIDTH) / 2 + c,
        })
    }
}
impl<W: io::Write> Drop for CrosstermRenderer<W> {
    fn drop(&mut self) {
        execute!(
            self.w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen,
        )
        .expect("Unable to drop CrosstermRenderer");

        terminal::disable_raw_mode().expect("Unable to drop CrosstermRenderer");
    }
}
impl<W: io::Write> Renderer for CrosstermRenderer<W> {
    fn clear(&mut self) {
        queue!(self.w, terminal::Clear(ClearType::All),).expect("Unable to clear terminal");
        let (c, r) = terminal::size().expect("Unable to determine size of terminal");
        self.term_size = Pos { r, c };
    }
    fn flush(&mut self) {
        self.w.flush().expect("Unable to flush writer");
    }

    fn render_str(&mut self, pos: Pos, text: &str) {
        if let Some(Pos { r, c }) = self.ui_offset(pos) {
            let _ = queue!(
                self.w,
                cursor::Hide,
                SetForegroundColor(Color::Rgb { r: 0, g: 255, b: 0 }),
            );

            for (i, l) in text.lines().enumerate() {
                let _ = queue!(self.w, cursor::MoveTo(c, r + i as u16), style::Print(l),);
            }
        } else {
            // Screen area too small..
            // TODO: Don't print it every time
            let _ = queue!(
                self.w,
                cursor::Hide,
                cursor::MoveTo(
                    std::cmp::max(self.term_size.c as i32 / 2 - 2, 0) as u16,
                    self.term_size.r / 2
                ),
                SetForegroundColor(Color::Rgb { r: 0, g: 255, b: 0 }),
                style::Print("^..^"),
            );
        }
    }
    fn render_fmt_str(&mut self, pos: Pos, text: &str, fmt: Style) {
        if let Some(Pos { r, c }) = self.ui_offset(pos) {
            let _ = queue!(self.w, cursor::Hide, cursor::MoveTo(c, r),);

            if fmt == Style::Invert {
                let _ = queue!(
                    self.w,
                    SetForegroundColor(Color::Black),
                    SetBackgroundColor(Color::Rgb { r: 0, g: 255, b: 0 }),
                    SetAttribute(Attribute::Bold),
                );
            }
            if fmt == Style::Highlight {
                let _ = queue!(
                    self.w,
                    SetForegroundColor(Color::Rgb { r: 0, g: 255, b: 0 }),
                    SetBackgroundColor(Color::Rgb { r: 0, g: 60, b: 0 }),
                );
            }
            for (i, l) in text.lines().enumerate() {
                let _ = queue!(self.w, cursor::MoveTo(c, r + i as u16), style::Print(l),);
            }

            if fmt != Style::Default {
                let _ = queue!(self.w, SetAttribute(Attribute::Reset), ResetColor,);
            }
        }
    }
}

pub struct CrosstermEventCollector {}
impl EventCollector for CrosstermEventCollector {
    fn poll_events(&self) -> Vec<crate::uifw::interaction::Event> {
        if !event::poll(Duration::from_secs(0)).unwrap() {
            // TODO handle error
            return vec![];
        }

        if let Ok(crossterm::event::Event::Key(key)) = event::read() {
            match key {
                KeyEvent {
                    code: KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                } => vec![crate::uifw::interaction::Event::Quit],
                KeyEvent {
                    code: KeyCode::Tab,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                } => vec![crate::uifw::interaction::Event::NextFocus],
                KeyEvent {
                    code: KeyCode::BackTab,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                } => vec![crate::uifw::interaction::Event::PrevFocus],
                KeyEvent {
                    code: KeyCode::Enter,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                } => vec![crate::uifw::interaction::Event::Activate],
                KeyEvent {
                    code: KeyCode::Delete,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                } => vec![crate::uifw::interaction::Event::Del],
                KeyEvent {
                    code: KeyCode::Backspace,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                } => vec![crate::uifw::interaction::Event::DelBack],
                KeyEvent {
                    code: KeyCode::Left,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                } => vec![crate::uifw::interaction::Event::Left],
                KeyEvent {
                    code: KeyCode::Right,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                } => vec![crate::uifw::interaction::Event::Right],
                KeyEvent {
                    code: KeyCode::Up,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                } => vec![crate::uifw::interaction::Event::Up],
                KeyEvent {
                    code: KeyCode::Down,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                } => vec![crate::uifw::interaction::Event::Down],
                KeyEvent {
                    code: KeyCode::Char(c),
                    kind: KeyEventKind::Press,
                    modifiers: modifier,
                    state,
                } => {
                    let mut cm = modifier.iter().fold(CharModifiers::None, |cm, km| {
                        cm | match km {
                            KeyModifiers::ALT => CharModifiers::Alt,
                            KeyModifiers::CONTROL => CharModifiers::Ctrl,
                            KeyModifiers::SHIFT => CharModifiers::Shift,
                            _ => CharModifiers::None,
                        }
                    });

                    cm = state.iter().fold(cm, |cm, state| {
                        cm | match state {
                            KeyEventState::CAPS_LOCK => CharModifiers::CapsLock,
                            _ => CharModifiers::None,
                        }
                    });

                    vec![crate::uifw::interaction::Event::Char(c, cm)]
                }
                _ => vec![],
            }
        } else {
            vec![]
        }
    }
}
