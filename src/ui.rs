use crate::ui::view::RootView;
use crate::ui::view::View;
use crate::ui::view_model::RootViewModel;
use std::io;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, style,
    terminal::{self},
};

use crossterm::event::KeyEventKind;

mod synth;
mod tracker;
pub mod view;
pub mod view_model;

pub struct UI {
    pub vm: RootViewModel,
    pub view: view::RootView,
}

impl UI {
    pub fn new() -> Self {
        let vm = RootViewModel::new();
        let view = RootView::new();
        UI { vm, view }
    }

    pub fn read_char() -> std::io::Result<char> {
        loop {
            if let Ok(Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) = event::read()
            {
                return Ok(c);
            }
        }
    }

    pub fn start(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        execute!(&mut stdout, terminal::EnterAlternateScreen)?;

        terminal::enable_raw_mode()?;

        loop {
            self.view.render(&self.vm, &mut stdout)?;

            match Self::read_char()? {
                'q' => {
                    execute!(&mut stdout, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                    break;
                }
                _ => {}
            };
        }

        execute!(
            stdout,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;

        terminal::disable_raw_mode()?;

        Ok(())
    }
}
