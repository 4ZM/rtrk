// Crossterm adapter for the interactions
use crate::interaction::EventCollector;
use crate::interaction::Renderer;
use crate::interaction::Style;
use crate::pos::Pos;
use crossterm::event::KeyEventKind;
use crossterm::style::ResetColor;
use std::io;
use std::time::Duration;

pub use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent},
    execute, queue, style,
    style::{Attribute, Color, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
};

pub struct CrosstermRenderer<W: io::Write> {
    w: W,
}
impl<W: io::Write> CrosstermRenderer<W> {
    pub fn new(mut w: W) -> CrosstermRenderer<W> {
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
            SetForegroundColor(Color::Rgb { r: 0, g: 255, b: 0 }),
        );

        for l in text.lines() {
            let _ = queue!(self.w, style::Print(l), cursor::MoveToNextLine(1));
        }
    }

    fn render_fmt_str(&mut self, Pos { r, c }: Pos, text: &str, fmt: Style) {
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

        for l in text.lines() {
            let _ = queue!(self.w, style::Print(l), cursor::MoveToNextLine(1));
        }

        if fmt != Style::Default {
            let _ = queue!(self.w, SetAttribute(Attribute::Reset), ResetColor,);
        }
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
            })) => return vec![crate::interaction::Event::NextFocus],
            Ok(crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::BackTab,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return vec![crate::interaction::Event::PrevFocus],
            Ok(crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return vec![crate::interaction::Event::Activate],
            Ok(crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Delete,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return vec![crate::interaction::Event::Del],
            Ok(crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return vec![crate::interaction::Event::DelBack],
            Ok(crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return vec![crate::interaction::Event::Left],
            Ok(crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) => return vec![crate::interaction::Event::Right],
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
