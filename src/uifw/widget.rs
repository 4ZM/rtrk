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

use crate::uifw::interaction::{Event, Renderer};
use crate::uifw::pos::Pos;

pub mod button;
pub mod focus;
pub mod label;
pub mod textbox;

/// A Widget is statefull and has the update() mechanism to mutate it's state.
/// It create views that are entirely disconnected from itself (no back ref with a lifetime).
pub trait Widget<Message, Task, V: View<Message>>: Focusable {
    fn update(&mut self, msg: Message) -> Vec<Task>;
    fn view(&self, pos: Pos) -> V;
}

/// A View can have data, but is stateless and immutable.  It's purpose is to interact with the
/// UI framework wrappers to draw and translate events to application messages.  Some things are
/// "just views" like labels (no state), some things like buttons are almost no state but are
/// focusable (i.e. have state)
pub trait View<Message> {
    fn on_event(&self, _e: Event) -> Vec<Message> {
        vec![]
    }
    fn draw(&self, _renderer: &mut dyn Renderer);
}

pub trait Focusable {
    /// Has focus directly or if any of it's children has focus
    fn has_focus(&self) -> bool;
    fn focus(&mut self);
    fn defocus(&mut self);

    /// Advance focus recursively
    fn next_focus(&mut self);
    fn prev_focus(&mut self);
}
