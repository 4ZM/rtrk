use crate::ui::view_model;
use crate::ui::view_model::Pos;
use crate::ui::view_model::ViewModel;
use std::io;

pub use crossterm::{
    cursor, queue, style,
    terminal::{self, ClearType},
};

pub trait View<W: io::Write> {
    fn render(&self, vm: &ViewModel, w: &mut W) -> io::Result<()>;
}

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

pub struct Tracks {
    pub track_list_pos: Pos,
    pub track_list_height: u8,
}

impl Tracks {
    pub fn new() -> Self {
        Tracks {
            track_list_pos: Pos { r: 14, c: 2 },
            track_list_height: 5,
        }
    }
}

impl<W: io::Write> View<W> for Tracks {
    fn render(&self, vm: &ViewModel, w: &mut W) -> io::Result<()> {
        let pos = &self.track_list_pos;
        queue!(w, cursor::MoveTo(pos.c, pos.r))?;

        let tl_active = vm.track_list_active;
        let tl_height = self.track_list_height;

        let tl_start = tl_active - tl_height / 2;
        let tl_end = tl_start + tl_height;

        // for i in tl_start..tl_end {
        //     self.render_sound(i, vm, w)?;
        //     queue!(w, cursor::MoveToColumn(pos.c), cursor::MoveDown(1))?;
        // }

        Ok(())
    }
}

pub struct Sound {
    pub sound_idx: u8,
}

impl<W: io::Write> View<W> for Sound {
    fn render(&self, vm: &ViewModel, w: &mut W) -> io::Result<()> {
        let snd = &vm.sounds[self.sound_idx as usize];

        // - -------- ------
        let wave_id = fmt_1(snd.wave_id);
        let attack = fmt_2(snd.attack);
        let decay = fmt_2(snd.decay);
        let sustain = fmt_2(snd.sustain);
        let release = fmt_2(snd.release);

        queue!(
            w,
            style::Print(&wave_id),
            cursor::MoveRight(1),
            style::Print(&attack),
            style::Print(&decay),
            style::Print(&sustain),
            style::Print(&release),
        )?;

        Ok(())
    }
}

pub struct SoundList {}

impl SoundList {
    const SOUNDS_LIST_HEIGHT: u8 = 6;
    const SOUNDS_LIST_POS: Pos = Pos { r: 2, c: 3 };
}

impl<W: io::Write> View<W> for SoundList {
    fn render(&self, vm: &ViewModel, w: &mut W) -> io::Result<()> {
        let pos = Self::SOUNDS_LIST_POS;

        let sl_start = vm.sounds_list_active;
        let sl_height = Self::SOUNDS_LIST_HEIGHT;

        queue!(w, cursor::MoveTo(pos.c, pos.r))?;

        for i in sl_start..sl_start + sl_height {
            queue!(w, style::Print(format!("{:02X}", i)), cursor::MoveRight(3))?;

            Sound { sound_idx: i }.render(vm, w)?;

            queue!(w, cursor::MoveToColumn(pos.c), cursor::MoveDown(1))?;
        }

        Ok(())
    }
}

pub struct Main {
    pub skin: &'static [&'static str],

    pub sound_list: SoundList,
    pub tracks: Tracks,
    pub version_pos: Pos,
    //pub track_spacing: u8,
}

impl<W: io::Write> View<W> for Main {
    fn render(&self, vm: &ViewModel, w: &mut W) -> io::Result<()> {
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

        self.sound_list.render(vm, w)?;
        self.tracks.render(vm, w)?;

        w.flush()?;

        Ok(())
    }
}

impl Main {
    pub fn new() -> Self {
        let split_skin = SKIN.lines().skip(1).collect::<Vec<_>>().leak();

        Main {
            skin: split_skin,
            version_pos: Pos { r: 0, c: 56 },
            sound_list: SoundList {},
            tracks: Tracks::new(),
            //track_spacing: 5,
        }
    }
}
