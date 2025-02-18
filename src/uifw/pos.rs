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

use std::ops;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Pos {
    pub r: u16,
    pub c: u16,
}

impl ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            r: self.r + rhs.r,
            c: self.c + rhs.c,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pos_math_test() {
        let p = Pos { r: 2, c: 3 };

        assert_eq!(p + Pos { r: 4, c: 2 }, Pos { r: 6, c: 5 });
    }
}
