use crate::ui::view::MainView;
use crate::ui::view::View;
use crate::ui::view_model::ViewModel;
use std::io;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, style,
    terminal::{self},
};

use crossterm::event::KeyEventKind;

mod view;
mod view_model;

pub struct Pos {
    pub r: u16,
    pub c: u16,
}

pub struct UI {
    pub vm: ViewModel,
    pub view: MainView,
}

impl UI {
    pub fn new() -> Self {
        let vm = ViewModel::new();
        let view = MainView::new();
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
