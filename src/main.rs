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
