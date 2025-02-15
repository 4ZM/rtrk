use std::io;
use std::io::Write;

use crossterm::event::KeyEventKind;
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command,
};

mod ui;

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

fn main() -> io::Result<()> {
    let ui = ui::UI::new();

    let mut stdout = io::stdout();

    execute!(&mut stdout, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    loop {
        ui.view.render(&ui.vm, &mut stdout)?;

        match read_char()? {
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
