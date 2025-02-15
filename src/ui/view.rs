use crate::ui::synth::view::SoundList;
use crate::ui::tracker::view::Tracks;
use crate::ui::view_model::RootViewModel;
use std::io;

pub use crossterm::{
    cursor, queue, style,
    terminal::{self, ClearType},
};

pub struct Pos {
    pub r: u16,
    pub c: u16,
}

pub trait View<W: io::Write> {
    fn render(&self, vm: &RootViewModel, w: &mut W) -> io::Result<()>;
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

pub fn fmt_1(val: Option<u8>) -> String {
    match val {
        Some(v) => format!("{:1X}", v),
        None => "-".to_string(),
    }
}

pub fn fmt_2(val: Option<u8>) -> String {
    match val {
        Some(v) => format!("{:02X}", v),
        None => "--".to_string(),
    }
}

pub struct RootView {
    pub skin: &'static [&'static str],

    pub sound_list: SoundList,
    pub tracks: Tracks,
    pub version_pos: Pos,
    //pub track_spacing: u8,
}

impl<W: io::Write> View<W> for RootView {
    fn render(&self, vm: &RootViewModel, w: &mut W) -> io::Result<()> {
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

impl RootView {
    pub fn new() -> Self {
        let split_skin = SKIN.lines().skip(1).collect::<Vec<_>>().leak();

        RootView {
            skin: split_skin,
            version_pos: Pos { r: 0, c: 56 },
            sound_list: SoundList {},
            tracks: Tracks::new(),
            //track_spacing: 5,
        }
    }
}
