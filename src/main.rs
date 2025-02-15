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
    let ui_layout = ui::Layout::new();

    let mut stdout = io::stdout();

    execute!(&mut stdout, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    loop {
        queue!(
            stdout,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        for line in ui_layout.skin.iter() {
            queue!(stdout, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        queue!(
            stdout,
            cursor::MoveTo(ui_layout.version_pos.c, ui_layout.version_pos.r),
            style::Print("5.1")
        )?;

        queue!(
            stdout,
            cursor::MoveTo(ui_layout.sound_code.c, ui_layout.sound_code.r),
            style::Print(format!("{:X}", 0x3)),
        )?;

        queue!(
            stdout,
            cursor::MoveTo(ui_layout.sound_list.c, ui_layout.sound_list.r),
            style::Print(format!("{:02X}", 0xF))
        )?;

        stdout.flush()?;

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
