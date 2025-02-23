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

mod app;
mod cycle;
mod synth;
mod uifw;

/// App -> Task -> Send [Synth Ctrl Channel] Recv -> Synth
/// Synt defines the channel and messages
/// App uses synt and translates task messages to synt messages

fn main() {
    let mut app = app::App::new();
    let mut task_processor = app::AppTaskProcessor::new();
    uifw::start(&mut app, &mut task_processor);
}
