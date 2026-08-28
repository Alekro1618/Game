use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    RGBA(RGBA),
}

impl Color {
    pub const BLACK: Self = Self::RGBA(RGBA::new(0.0, 0.0, 0.0, 1.0));
    pub const NONE: Self = Self::RGBA(RGBA::new(0.0, 0.0, 0.0, 0.0));
    pub const WHITE: Self = Self::RGBA(RGBA::new(1.0, 1.0, 1.0, 1.0));
    pub const RED: Self = Self::RGBA(RGBA::new(1.0, 0.0, 0.0, 1.0));
    pub const GREEN: Self = Self::RGBA(RGBA::new(0.0, 1.0, 0.0, 1.0));
    pub const BLUE: Self = Self::RGBA(RGBA::new(0.0, 0.0, 1.0, 1.0));
    pub const YELLOW: Self = Self::RGBA(RGBA::new(1.0, 1.0, 0.0, 1.0));
    pub const PURPLE: Self = Self::RGBA(RGBA::new(1.0, 0.0, 1.0, 1.0));
    pub const CYAN: Self = Self::RGBA(RGBA::new(0.0, 1.0, 1.0, 1.0));

    pub fn set_alpha(&mut self, alpha : f32) {
        match self {
            Color::RGBA(rgba) => {
                rgba.a = alpha
            },
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl From<&[char]> for Color {
    fn from(value: &[char]) -> Self {
        if value[0] != '#' {
            panic!("Missing # in color parameter");
        }
        todo!("")
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl RGBA {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn hex(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    pub fn average(&self, rhs: &Self) -> Self {
        Self {
            r: (self.r + rhs.r) / 2.0,
            g: (self.g + rhs.g) / 2.0,
            b: (self.b + rhs.b) / 2.0,
            a: (self.a + rhs.a) / 2.0,
        }
    }
}

impl Add for RGBA {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl Mul for RGBA {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

pub struct Gradient {
    splits: Vec<RGBA>,
}

impl Gradient {
    pub fn new(colors: impl Iterator<Item = RGBA>, divisions: u32) -> Self {
        let mut this = Self {
            splits: colors.collect(),
        };
        this.subdivide(divisions);
        this
    }

    pub fn subdivide(&mut self, times: u32) {
        if times == 0 {
            return;
        }

        self.splits = self
            .splits
            .windows(2)
            .map(|window| window[0].average(&window[1]))
            .collect();
        self.subdivide(times - 1);
    }
}
