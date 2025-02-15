use crate::ui::view::Pos;
use crate::ui::view::View;
use crate::ui::view_model::RootViewModel;
use std::io;

pub use crossterm::{cursor, queue};

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
    fn render(&self, vm: &RootViewModel, w: &mut W) -> io::Result<()> {
        let pos = &self.track_list_pos;
        queue!(w, cursor::MoveTo(pos.c, pos.r))?;

        let tl_active = vm.tracks.track_list_active;
        let tl_height = self.track_list_height;

        let tl_start = tl_active - tl_height / 2;
        let tl_end = tl_start + tl_height;

        for _i in tl_start..tl_end {
            //            self.render_sound(i, vm, w)?;
            queue!(w, cursor::MoveToColumn(pos.c), cursor::MoveDown(1))?;
        }

        Ok(())
    }
}
