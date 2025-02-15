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
┃  00 > - -------- ------             \         \   ____/       \   :/    /    ┃
┃  00 . - -------- ------             /    <    /:  \ \    >    /   ;   _/     ┃
┃  00 . - -------- ------            /         < |   \/       <<         \     ┃
┃  00 : - -------- ------           /      :    \|    \    ;    \   ,     \    ┃
┃  00 ' - -------- ------           \      |     \    /    |     \  :      \   ┃
┃  00 , - -------- ------            \  ___^_____/   /\____|     /__:       \  ┃
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

#[derive(Copy, Clone, Default)]
pub struct Sound {
    wave_id: Option<u8>,
    attack: Option<u8>,
    decay: Option<u8>,
    sustain: Option<u8>,
    release: Option<u8>,
}

impl Sound {
    fn new() -> Sound {
        Sound {
            wave_id: None,
            attack: None,
            decay: None,
            sustain: None,
            release: None,
        }
    }
}

pub struct ViewModel {
    sounds: [Sound; 255],

    sounds_list_active: u8,
    track_list_active: u8,

    version: String,
}

impl ViewModel {
    pub fn new() -> Self {
        let mut sounds = [Sound::new(); 255];

        let demo_sound_1 = Sound {
            wave_id: Some(2),
            attack: Some(0x35),
            decay: Some(0xFF),
            sustain: Some(0x10),
            release: Some(0x01),
        };
        let demo_sound_2 = Sound {
            wave_id: Some(4),
            attack: Some(0x10),
            decay: Some(0x10),
            sustain: Some(0xA0),
            release: Some(0x00),
        };

        // DUMMY DATA TBD
        sounds[2] = demo_sound_1;
        sounds[4] = demo_sound_2;

        ViewModel {
            version: Self::version(),
            sounds,
            sounds_list_active: 1, // TBD Demo value, should start at 0
            track_list_active: 5,  // TBD Demo value, should start at 0
        }
    }

    pub fn tic(dt_s: f32) {
        // Advance UI state
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

    pub sound_list_pos: Pos,
    pub sounds_list_height: u8,

    pub track_list_pos: Pos,
    pub track_list_height: u8,
    pub track_spacing: u8,
}

fn fmt_1(val: Option<u8>) -> String {
    match val {
        Some(v) => format!("{:1X}", v),
        None => "-".to_string(),
    }
}

fn fmt_2(val: Option<u8>) -> String {
    match val {
        Some(v) => format!("{:02X}", v),
        None => "--".to_string(),
    }
}

impl View {
    pub fn new() -> Self {
        let split_skin = SKIN.lines().skip(1).collect::<Vec<_>>().leak();

        View {
            skin: split_skin,
            version_pos: Pos { r: 0, c: 56 },
            sound_list_pos: Pos { r: 2, c: 3 },
            sounds_list_height: 6,
            track_list_pos: Pos { r: 14, c: 2 },
            track_list_height: 5,
            track_spacing: 5,
        }
    }

    fn render_tracks<W>(&self, vm: &ViewModel, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        let pos = &self.track_list_pos;
        queue!(w, cursor::MoveTo(pos.c, pos.r))?;

        let tl_active = vm.track_list_active;
        let tl_height = self.track_list_height;

        let tl_start = tl_active - tl_height / 2;
        let tl_end = tl_start + tl_height;

        for i in tl_start..tl_end {
            // self.render_sound(i, vm, w)?;
            queue!(w, cursor::MoveToColumn(pos.c), cursor::MoveDown(1))?;
        }

        Ok(())
    }

    fn render_sound_list<W>(&self, vm: &ViewModel, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        let pos = &self.sound_list_pos;

        let sl_start = vm.sounds_list_active;
        let sl_height = self.sounds_list_height;

        queue!(w, cursor::MoveTo(pos.c, pos.r))?;
        for i in sl_start..sl_start + sl_height {
            self.render_sound(i, vm, w)?;
            queue!(w, cursor::MoveToColumn(pos.c), cursor::MoveDown(1))?;
        }

        Ok(())
    }

    fn render_sound<W>(&self, snd_idx: u8, vm: &ViewModel, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        let snd = &vm.sounds[snd_idx as usize];

        // 00 . - -------- ------
        let wave_id = fmt_1(snd.wave_id);
        let attack = fmt_2(snd.attack);
        let decay = fmt_2(snd.decay);
        let sustain = fmt_2(snd.sustain);
        let release = fmt_2(snd.release);

        queue!(
            w,
            style::Print(format!("{:02X}", snd_idx)),
            cursor::MoveRight(3),
            style::Print(&wave_id),
            cursor::MoveRight(1),
            style::Print(&attack),
            style::Print(&decay),
            style::Print(&sustain),
            style::Print(&release),
        )?;

        Ok(())
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

        self.render_sound_list(vm, w)?;
        self.render_tracks(vm, w)?;

        w.flush()?;

        Ok(())
    }
}
