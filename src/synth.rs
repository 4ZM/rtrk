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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Oscilator {
    Sine,
    Triangle,
    Saw,
    Square,
    Pulse,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Envelope {
    ///  |  '
    ///  | / \_________      < A
    ///  |/            \
    ///  |______________\____
    ///  ^  ^^        ^ ^
    ///  0  12        3 4
    ///
    //pub attack_lvl: f32, // Always 0->100%
    pub attack_ms: f32, // [0,1]
    // pub decay_lvl: f32, // Always 100% -> A
    pub decay_ms: f32,    // [1,2]
    pub sustain_lvl: f32, // A
    // pub sustain_min_ms: f32,  // Depends on hos long it's played
    // pub release_lvl: f32, // Always A -> 0
    pub release_ms: f32, // [3,4]
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            //attack_lvl: 1.0,
            attack_ms: 0.1,
            //decay_lvl: 0.9,
            decay_ms: 0.1,
            sustain_lvl: 0.9,
            //sustain_min_ms: 0.5,
            //release_lvl: 0.0,
            release_ms: 0.1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Filter {
    pub cutoff: f32,
    pub gain: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Voice {
    pub osc: Oscilator,
    pub env: Option<Envelope>,
    pub lp: Option<Filter>,
    pub hp: Option<Filter>,
}

use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::time::Duration;

const SAMPLE_RATE: u32 = 44100;

mod math;
mod wave_tables;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Frequency(pub f32);
pub struct Note;

#[allow(dead_code)]
impl Note {
    pub const C: Frequency = Frequency(261.63);
    pub const CS: Frequency = Frequency(277.18);
    pub const D: Frequency = Frequency(293.66);
    pub const DS: Frequency = Frequency(311.13);
    pub const E: Frequency = Frequency(329.63);
    pub const F: Frequency = Frequency(349.23);
    pub const FS: Frequency = Frequency(369.99);
    pub const G: Frequency = Frequency(392.0);
    pub const GS: Frequency = Frequency(415.3);
    pub const A: Frequency = Frequency(440.0);
    pub const AS: Frequency = Frequency(466.16);
    pub const B: Frequency = Frequency(493.88);
}

struct WaveTableOscillator {
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
    remaining_samples: u32,
    interpolator: math::Interpolator,
}

impl WaveTableOscillator {
    fn new(
        wave_table: Vec<f32>,
        interpolator: math::Interpolator,
        duration_sec: f32, // TODO should be Remaining duration (or None if infinite)
    ) -> WaveTableOscillator {
        let wave_table_len = wave_table.len();
        return WaveTableOscillator {
            wave_table,
            index: 0.0,
            index_increment: Note::A.0 * wave_table_len as f32 / SAMPLE_RATE as f32,
            remaining_samples: (SAMPLE_RATE as f32 * duration_sec) as u32,
            interpolator,
        };
    }

    fn get_sample(&mut self) -> f32 {
        let sample = (self.interpolator)(&self.wave_table, self.index);
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        sample
    }

    fn set_frequency(&mut self, Frequency(freq_hz): Frequency) {
        self.index_increment = freq_hz * self.wave_table.len() as f32 / SAMPLE_RATE as f32;
    }
}

impl Iterator for WaveTableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.remaining_samples > 0 {
            self.remaining_samples -= 1;
            Some(self.get_sample())
        } else {
            None
        }
    }
}

impl Source for WaveTableOscillator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

use std::sync::mpsc::{self, SendError};
use std::thread;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Message {
    Play(Voice, usize, Frequency),
    Terminate,
}
struct AsyncSynth {
    thread: Option<thread::JoinHandle<()>>,
    tx: mpsc::Sender<Message>,
}
impl AsyncSynth {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let thread = Some(thread::spawn(move || {
            let mut synth = Synth::new();

            loop {
                let message = rx.recv();

                match message {
                    Ok(Message::Play(voice, channel, freq)) => {
                        synth.play(channel, &voice, freq, 1.0)
                    }
                    Ok(Message::Terminate) => break,
                    Err(_) => break,
                }
            }
        }));

        Self { thread, tx }
    }
    pub fn send(&mut self, msg: Message) -> Result<(), SendError<Message>> {
        self.tx.send(msg)
    }

    pub fn drop(&mut self) {
        let _ = self.send(Message::Terminate); // OK if this fails if thread already finished
        self.thread
            .take()
            .unwrap()
            .join()
            .expect("Synth thread panic!");
    }
}

pub struct Synth {
    stream: OutputStream, // Keep stream alive, can't use just the handle
    stream_handle: OutputStreamHandle,
    channels: Vec<Sink>,
}

impl Synth {
    pub fn new() -> Self {
        let (stream, stream_handle) =
            OutputStream::try_default().expect("Could not use default audio device.");
        let channels = (0..4)
            .map(|_| Sink::try_new(&stream_handle).expect("Could not create audio sink"))
            .collect();
        Self {
            stream,
            stream_handle,
            channels,
        }
    }
    pub fn play(&mut self, channel: usize, voice: &Voice, freq_hz: Frequency, _duration_ms: f32) {
        let mut osc = match voice.osc {
            Oscilator::Sine => WaveTableOscillator::new(wave_tables::sine(32), math::lerp, 1.0),
            Oscilator::Triangle => {
                WaveTableOscillator::new(wave_tables::triangle(), math::lerp, 1.0)
            }
            Oscilator::Saw => WaveTableOscillator::new(wave_tables::saw(32), math::lerp, 1.0),
            Oscilator::Square => WaveTableOscillator::new(wave_tables::square(), math::step, 1.0),
            Oscilator::Pulse => {
                WaveTableOscillator::new(wave_tables::pulse(64, 0.1), math::step, 1.0)
            }
        };
        osc.set_frequency(freq_hz);

        self.channels[channel].clear();
        self.channels[channel].append(osc);
        self.channels[channel].play();
    }

    pub fn wait(&self) {
        for channel in self.channels.iter() {
            channel.sleep_until_end();
        }
    }

    pub fn drop(&mut self) {
        for channel in self.channels.iter() {
            channel.clear();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::thread;

    // #[test]
    // fn async_synth_test() {
    //     let voice = Voice {
    //         osc: Oscilator::Triangle,
    //         env: None,
    //         lp: None,
    //         hp: None,
    //     };
    //     let mut synth = AsyncSynth::new();

    //     synth.send(Message::Play(voice, 0, Note::A)).expect("");
    //     synth.send(Message::Play(voice, 2, Note::C)).expect("");

    //     thread::sleep(Duration::from_secs(1));

    //     synth.send(Message::Terminate).expect("");
    //     synth.drop();
    // }

    // #[test]
    // fn polyphony_test() {
    //     let voice = Voice {
    //         osc: Oscilator::Triangle,
    //         env: None,
    //         lp: None,
    //         hp: None,
    //     };
    //     let mut synth = Synth::new();

    //     let duration_sec = 1.0;
    //     synth.play(0, &voice, Note::A, duration_sec);
    //     synth.play(1, &voice, Note::C, duration_sec);
    //     synth.wait();
    // }
}
