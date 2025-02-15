use crate::ui;
use crate::ui::view::Pos;
use crate::ui::view::View;
use crate::ui::view_model::RootViewModel;
use std::io;

pub use crossterm::{cursor, queue, style};

pub struct Sound {
    pub sound_idx: u8,
}

impl<W: io::Write> View<W> for Sound {
    fn render(&self, vm: &RootViewModel, w: &mut W) -> io::Result<()> {
        let snd = &vm.synth.sounds[self.sound_idx as usize];

        // - -------- ------
        let wave_id = ui::view::fmt_1(snd.wave_id);
        let attack = ui::view::fmt_2(snd.attack);
        let decay = ui::view::fmt_2(snd.decay);
        let sustain = ui::view::fmt_2(snd.sustain);
        let release = ui::view::fmt_2(snd.release);

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
    fn render(&self, vm: &RootViewModel, w: &mut W) -> io::Result<()> {
        let pos = Self::SOUNDS_LIST_POS;

        let sl_start = vm.synth.sounds_list_active;
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
