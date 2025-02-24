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

use crate::synth::{AudioSink, WaveTableOscillator, SAMPLE_RATE};
use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

use std::marker::PhantomData;
pub struct RodioAudioSink<T> {
    _marker: PhantomData<T>,
    _stream: OutputStream, // Keep stream alive, can't use just the handle
    channels: Vec<Sink>,
}
impl<T> RodioAudioSink<T> {
    pub fn new(n_channels: usize) -> Self {
        let (_stream, stream_handle) =
            OutputStream::try_default().expect("Could not use default audio device.");
        let channels = (0..n_channels)
            .map(|_| Sink::try_new(&stream_handle).expect("Could not create audio sink"))
            .collect();
        Self {
            _marker: PhantomData,
            _stream,
            channels,
        }
    }
}
impl<T> AudioSink for RodioAudioSink<T>
where
    T: Iterator<Item = f32> + Source + Send + 'static,
{
    //type Iter = WaveTableOscillator;
    type Iter = T; //Iterator<Item = f32> + Source + Send + 'static;
    fn play(&mut self, channel: usize, data: Self::Iter) {
        self.channels[channel].clear();
        self.channels[channel].append(data);
        self.channels[channel].play();
    }

    fn stop(&mut self, channel: usize) {
        self.channels[channel].clear();
    }

    fn wait(&mut self, channel: usize) {
        self.channels[channel].sleep_until_end();
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
