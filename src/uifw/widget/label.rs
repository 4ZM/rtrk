// Copyright (C) 2025 Anders Sundman <anders@4zm.org>
//
// This file is part of RTRK - The Rust Tracker
//
// RTRK is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// RTRK is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with RTRK. If not, see <https://www.gnu.org/licenses/>.

use crate::uifw::pos::Pos;
use crate::uifw::widget::View;

pub struct Label {
    pos: Pos,
    text: String,
}

impl View<()> for Label {
    fn draw(&self, renderer: &mut dyn crate::uifw::interaction::Renderer) {
        renderer.render_str(self.pos, &format!("{}", &self.text));
    }
}
pub fn label(pos: Pos, text: &str) -> Label {
    Label {
        pos,
        text: text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uifw::interaction::{tests::TestRenderer, Event};

    #[test]
    fn label_test() {
        let lbl = label(Pos { r: 0, c: 0 }, "LBL");

        let mut renderer = TestRenderer::new();
        lbl.draw(&mut renderer);
        assert_eq!(renderer.out, "LBL");

        // Can't activate a label
        let msg = lbl.on_event(Event::Activate);
        assert!(msg.is_empty());
    }
}
