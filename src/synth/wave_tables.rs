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

pub fn sine(samples: usize) -> Vec<f32> {
    (0..samples)
        .map(|i| (std::f32::consts::TAU * i as f32 / samples as f32).sin())
        .collect()
}
pub fn triangle() -> Vec<f32> {
    vec![0.0, 1.0, 0.0, -1.0]
}

pub fn square() -> Vec<f32> {
    vec![1.0, -1.0]
}

pub fn saw(samples: usize) -> Vec<f32> {
    let mut table = vec![0.0; samples];
    let k = 2.0 / samples as f32;
    for n in 0..samples / 2 {
        table[n] = k * n as f32;
    }
    for n in samples / 2..samples {
        table[n] = k * n as f32;
    }
    table
}

pub fn pulse(samples: usize, duty_cycle: f32) -> Vec<f32> {
    let mut table = vec![1.0; samples];
    let flip_index = (duty_cycle * samples as f32) as usize;

    for i in flip_index..samples {
        table[i] = -1.0;
    }

    table
}
