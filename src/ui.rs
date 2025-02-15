use std::io;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
};
use itertools::Itertools;

use crossterm::event::KeyEventKind;

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

pub struct Pos {
    pub r: u16,
    pub c: u16,
}

pub struct UI {
    pub vm: ViewModel,
    pub view: View,
}

impl UI {
    pub fn new() -> Self {
        let vm = ViewModel::new();
        let view = View::new();
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

pub struct ViewModel {
    version: String,
}

impl ViewModel {
    pub fn new() -> Self {
        ViewModel {
            version: Self::version(),
        }
    }

    fn version() -> String {
        env!("CARGO_PKG_VERSION")
            .split('.')
            .take(2)
            .join(".")
            .to_string()
    }
}

pub struct View {
    pub skin: &'static [&'static str],
    pub version_pos: Pos,
    pub sound_list: Pos,
    pub sound_code: Pos,
}

impl View {
    pub fn new() -> Self {
        let split_skin = SKIN.lines().skip(1).collect::<Vec<_>>().leak();

        View {
            skin: split_skin,
            version_pos: Pos { r: 0, c: 56 },
            sound_list: Pos { r: 2, c: 3 },
            sound_code: Pos { r: 2, c: 8 },
        }
    }

    pub fn render<W>(&self, vm: &ViewModel, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        for line in self.skin.iter() {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        queue!(
            w,
            cursor::MoveTo(self.version_pos.c, self.version_pos.r),
            style::Print(&vm.version)
        )?;

        queue!(
            w,
            cursor::MoveTo(self.sound_code.c, self.sound_code.r),
            style::Print(format!("{:X}", 0x3)),
        )?;

        queue!(
            w,
            cursor::MoveTo(self.sound_list.c, self.sound_list.r),
            style::Print(format!("{:02X}", 0xF))
        )?;

        w.flush()?;

        Ok(())
    }
}
