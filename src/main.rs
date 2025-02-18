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

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod app;
mod cycle;
mod synth;
mod uifw;

/// App -> Task -> Send [Synth Ctrl Channel] Recv -> Synth
/// Synt defines the channel and messages
/// App uses synt and translates task messages to synt messages
use synth::*;

fn main() {
    let mut app = app::App::new();
    uifw::start(&mut app);

    // let voice = Voice {
    //     osc: Oscilator::Pulse,
    //     env: None,
    //     lp: None,
    //     hp: None,
    // };
    // let synth = Synth {};

    // let duration_sec = 1.0;
    // synth.play(&voice, Note::A, duration_sec);
    // thread::sleep(Duration::from_millis(duration_sec as u64 * 1000));

    // let ct = Arc::new(AtomicBool::new(false));
    // let ct_thread = Arc::clone(&ct);

    // let (tx, rx) = mpsc::channel();
    // let t = thread::spawn(move || {
    //     let vals = vec!["first", "and", "last", "and", "always"];

    //     for val in vals.iter().cycle() {
    //         if ct_thread.load(Ordering::Relaxed) {
    //             break;
    //         }
    //         tx.send(String::from(*val)).unwrap();
    //         thread::sleep(Duration::from_millis(1000));
    //     }
    // });

    // for _n in 0..5 {
    //     let recieved = rx.recv();
    //     println!("Received: {recieved:?}");
    // }

    // println!("Requesting termination");
    // ct.store(true, Ordering::Relaxed);
    // let _ = t.join();
    // println!("Joined");
}
