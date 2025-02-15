use std::marker::Copy;
use std::ops::{Add, AddAssign, Deref, Sub, SubAssign};

#[derive(Copy, Clone, Debug)]
pub struct Cycle {
    n: usize,
    max: usize,
}

impl Cycle {
    pub fn new(n: usize, max: usize) -> Self {
        Self { n, max }
    }
    pub fn _set(&mut self, n: usize) {
        self.n = n % self.max;
    }
}
impl Deref for Cycle {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.n
    }
}

impl Add<usize> for Cycle {
    type Output = Self;
    fn add(self, other: usize) -> Self::Output {
        Self {
            n: (self.n + other) % self.max,
            max: self.max,
        }
    }
}
impl Sub<usize> for Cycle {
    type Output = Self;
    fn sub(self, other: usize) -> Self::Output {
        Self {
            n: (self.n + self.max - other) % self.max,
            max: self.max,
        }
    }
}

impl AddAssign<usize> for Cycle {
    fn add_assign(&mut self, other: usize) {
        self.n = (self.n + other) % self.max;
    }
}
impl SubAssign<usize> for Cycle {
    fn sub_assign(&mut self, other: usize) {
        self.n = (self.n + self.max - other) % self.max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_test() {
        let c = Cycle { n: 1, max: 2 };
        assert_eq!(*c, 1);
    }

    #[test]
    fn arithmetic_test() {
        let c = Cycle { n: 1, max: 2 };
        assert_eq!(*(c + 0), 1);
        assert_eq!(*(c + 1), 0);
        assert_eq!(*(c + 2), 1);
        assert_eq!(*(c - 1), 0);
        assert_eq!(*(c - 2), 1);

        let c = Cycle { n: 0, max: 5 };
        assert_eq!(*(c + 0), 0);
        assert_eq!(*(c + 1), 1);
        assert_eq!(*(c + 4), 4);
        assert_eq!(*(c + 5), 0);
    }
    #[test]
    fn arithmetic_assign_test() {
        let mut c = Cycle { n: 0, max: 4 };
        c += 1;
        assert_eq!(*c, 1);
        c += 2;
        assert_eq!(*c, 3);
        c += 1;
        assert_eq!(*c, 0);

        let mut c = Cycle { n: 1, max: 4 };
        c -= 1;
        assert_eq!(*c, 0);
        c -= 1;
        assert_eq!(*c, 3);
        c -= 2;
        assert_eq!(*c, 1);
    }

    #[test]
    fn assign_test() {
        let mut c = Cycle { n: 1, max: 4 };
        assert_eq!(*c, 1);
        c._set(2);
        assert_eq!(*c, 2);
        c._set(7);
        assert_eq!(*c, 3);
    }
}
