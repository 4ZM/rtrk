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

mod ui {
    pub struct Pos {
        pub r: u16,
        pub c: u16,
    }

    pub struct Layout {
        pub skin: &'static [&'static str],
        pub version_pos: Pos,
        pub sound_list: Pos,
        pub sound_code: Pos,
    }

    const SKIN: &str = r#"
┏━━━━━━━━━[ rtrk ]━━━━━━━━━━━━━━━━━━━━━━━━━━ , ━━━━━━ [v . ] ━━━ =^..^= ━━━━━━━┓
┃                                     ______/ \_ _/\______,___/\ ___' _____,   ┃
┃  00 . - -------- ------             \         \   ____/       \   :/    /    ┃
┃  00 : - -------- ------             /    <    /:  \ \    >    /   ;   _/     ┃
┃  00 : - -------- ------            /         < |   \/       <<         \     ┃
┃  00 : - -------- ------           /      :    \|    \    ;    \   ,     \    ┃
┃  00 : - -------- ------           \      |     \    /    |     \  :      \   ┃
┃  00 ' - -------- ------            \  ___^_____/   /\____|     /__:       \  ┃
┃                                     \/   ;      \ /  4ZM  \___/   |_______/  ┃
┠──────────────────────────────────────────────────'───────────────────────────┨
┃ ▚▚▚▚▚▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚█████                                                  ┃
┠──────────────────────────────────────────────────────────────────────────────┨
┃ ## :               :                :                :                :  gFx ┃
┠──────────────────────────────────────────────────────────────────────────────┨
┃ 00 . --- - -- ---  .  --- - -- ---  .  --- - -- ---  .  --- - -- ---  .  --- ┃
┃ 00 : --- - -- ---  :  --- - -- ---  :  --- - -- ---  :  --- - -- ---  :  --- ┃
┃ 00 > --- - -- --- <:> --- - -- --- <:> --- - -- --- <:> --- - -- --- <:> --- ┃
┃ 00 : --- - -- ---  :  --- - -- ---  :  --- - -- ---  :  --- - -- ---  :  --- ┃
┃ 00 ' --- - -- ---  '  --- - -- ---  '  --- - -- ---  '  --- - -- ---  '  --- ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
"#;

    impl Layout {
        pub fn new() -> Layout {
            let split_skin = SKIN.lines().skip(1).collect::<Vec<_>>().leak();

            Layout {
                skin: split_skin,
                version_pos: Pos { r: 0, c: 56 },
                sound_list: Pos { r: 2, c: 3 },
                sound_code: Pos { r: 2, c: 8 },
            }
        }
    }
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
