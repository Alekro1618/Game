use std::{
    fmt::Display,
    ops::{Add, AddAssign, BitOr, BitXor, Mul, Neg, Sub},
};

use serde::{Deserialize, Serialize};

pub fn vec2(x: impl Into<f32>, y: impl Into<f32>) -> Vec2 {
    Vec2::new(x.into(), y.into())
}



#[derive(Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let strings = format!("({:.2}, {:.2})", self.x, self.y);
        f.write_str(&strings)
    }
}

impl Vec2 {
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All `f32::MIN`.
    pub const MIN: Self = Self::splat(f32::MIN);

    /// All `f32::MAX`.
    pub const MAX: Self = Self::splat(f32::MAX);

    /// All `f32::NAN`.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// All `f32::INFINITY`.
    pub const INFINITY: Self = Self::splat(f32::INFINITY);

    /// All `f32::NEG_INFINITY`.
    pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0);

    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0);

    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0);

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }

    #[inline]
    pub fn norm_sq(&self) -> f32 {
        self ^ self
    }

    #[inline]
    pub fn norm(&self) -> f32 {
        //TODO: REMAKE
        self.norm_sq().sqrt()
    }

    #[inline]
    pub fn norm_inv(&self) -> f32 {
        //TODO: REMAKE
        self.norm_sq().sqrt().recip()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        self.norm_inv() * self
    }

    #[inline]
    pub const fn from_array(a: [f32; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    #[inline]
    pub fn distance(self, from: Vec2) -> f32 {
        (self - from).norm()
    }

    #[inline]
    pub fn distance_sq(self, from: Vec2) -> f32 {
        (self - from).norm_sq()
    }

    #[inline]
    pub fn distance_inv(self, from: Vec2) -> f32 {
        (self - from).norm_inv()
    }

    #[inline]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        (1.0 - s) * self + s * rhs
    }

    #[inline]
    pub fn mult_by(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }

    #[inline]
    pub fn cos_angle(&self, rhs: &Self) -> f32 {
        self.normalize() ^ rhs.normalize()
    }

    #[inline]
    pub fn sin_angle(&self, rhs: &Self) -> f32 {
        self.normalize() | rhs.normalize()
    }

    #[inline]
    pub fn map(self, action: impl Fn(f32) -> f32) -> Self {
        Self::new(action(self.x), action(self.y))
    }

    #[inline]
    pub fn min_element(self) -> f32 {
        let min = |a, b| if a < b { a } else { b };
        min(self.x, self.y)
    }

    #[inline]
    pub fn max_element(self) -> f32 {
        let max = |a, b| if a > b { a } else { b };
        max(self.x, self.y)
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x - rhs.y * self.y,
            y: self.x * rhs.y + rhs.x * self.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl BitXor for &Vec2 {
    type Output = f32;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl BitXor for Vec2 {
    type Output = f32;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl BitOr for Vec2 {
    type Output = f32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl Into<Vec2> for (f32, f32) {
    fn into(self) -> Vec2 {
        Vec2::new(self.0, self.1)
    }
}
