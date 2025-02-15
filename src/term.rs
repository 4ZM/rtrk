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
