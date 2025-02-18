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
