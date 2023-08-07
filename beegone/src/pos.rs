use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Pos {
    pub q: i32,
    pub r: i32,
}

impl Pos {
    pub fn adjacent(self) -> impl Iterator<Item = Pos> {
        Shift::directions().map(move |shift| self + shift)
    }

    pub fn s(&self) -> i32 {
        -self.q - self.r
    }
}

impl Sub for Pos {
    type Output = Shift;

    fn sub(self, rhs: Self) -> Self::Output {
        Shift {
            dq: self.q - rhs.q,
            dr: self.r - rhs.r,
        }
    }
}

impl Add<Shift> for Pos {
    type Output = Pos;

    fn add(self, rhs: Shift) -> Self::Output {
        Pos {
            q: self.q + rhs.dq,
            r: self.r + rhs.dr,
        }
    }
}

impl AddAssign<Shift> for Pos {
    fn add_assign(&mut self, rhs: Shift) {
        self.q += rhs.dq;
        self.r += rhs.dr;
    }
}

impl Sub<Shift> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Shift) -> Self::Output {
        Pos {
            q: self.q - rhs.dq,
            r: self.r - rhs.dr,
        }
    }
}

impl SubAssign<Shift> for Pos {
    fn sub_assign(&mut self, rhs: Shift) {
        self.q -= rhs.dq;
        self.r -= rhs.dr;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Shift {
    pub dq: i32,
    pub dr: i32,
}

impl Shift {
    pub const UP: Shift = Shift { dq: 0, dr: -1 };
    pub const UP_RIGHT: Shift = Shift { dq: 1, dr: -1 };
    pub const DOWN_RIGHT: Shift = Shift { dq: 1, dr: 0 };
    pub const DOWN: Shift = Shift { dq: 0, dr: 1 };
    pub const DOWN_LEFT: Shift = Shift { dq: -1, dr: 1 };
    pub const UP_LEFT: Shift = Shift { dq: -1, dr: 0 };

    const DIRECTIONS: [Shift; 6] = [
        Self::UP,
        Self::UP_RIGHT,
        Self::DOWN_RIGHT,
        Self::DOWN,
        Self::DOWN_LEFT,
        Self::UP_LEFT,
    ];

    #[inline]
    fn ds(&self) -> i32 {
        -self.dq - self.dr
    }

    pub fn directions() -> impl Iterator<Item = Shift> {
        Self::DIRECTIONS.into_iter()
    }

    pub fn distance(&self) -> i32 {
        self.dq.abs() + self.dr.abs() + self.ds().abs()
    }
}

impl Neg for Shift {
    type Output = Shift;

    fn neg(self) -> Self::Output {
        Shift {
            dq: -self.dq,
            dr: -self.dr,
        }
    }
}

impl Add for Shift {
    type Output = Shift;

    fn add(self, rhs: Self) -> Self::Output {
        Shift {
            dq: self.dq + rhs.dq,
            dr: self.dr + rhs.dr,
        }
    }
}

impl AddAssign for Shift {
    fn add_assign(&mut self, rhs: Self) {
        self.dq += rhs.dq;
        self.dr += rhs.dr;
    }
}

impl Sub for Shift {
    type Output = Shift;

    fn sub(self, rhs: Self) -> Self::Output {
        Shift {
            dq: self.dq - rhs.dq,
            dr: self.dr - rhs.dr,
        }
    }
}

impl SubAssign for Shift {
    fn sub_assign(&mut self, rhs: Self) {
        self.dq -= rhs.dq;
        self.dr -= rhs.dr;
    }
}

impl Mul<i32> for Shift {
    type Output = Shift;

    fn mul(self, rhs: i32) -> Self::Output {
        Shift {
            dq: self.dq * rhs,
            dr: self.dr * rhs,
        }
    }
}

impl Mul<Shift> for i32 {
    type Output = Shift;

    fn mul(self, rhs: Shift) -> Self::Output {
        Shift {
            dq: self * rhs.dq,
            dr: self * rhs.dr,
        }
    }
}

impl MulAssign<i32> for Shift {
    fn mul_assign(&mut self, rhs: i32) {
        self.dq *= rhs;
        self.dr *= rhs;
    }
}
