use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug)]
pub struct ColorRBG {
    r: f64,
    g: f64,
    b: f64,
}

impl ColorRBG {
    pub fn new(r: f64, g: f64, b: f64) -> ColorRBG {
        ColorRBG { r, g, b }
    }

    pub fn get_value(&self) -> (f64, f64, f64) {
        (self.r, self.g, self.b)
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }

    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };
    pub const YELLOW: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 0.0,
    };
    pub const PINK: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 1.0,
    };
    pub const TURQUOISE: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 1.0,
    };
    pub const ORANGE: Self = Self {
        r: 1.0,
        g: 0.5,
        b: 0.0,
    };
    pub const LIGHT_BLUE: Self = Self {
        r: 0.0,
        g: 0.5,
        b: 1.0,
    };
}

impl Add for ColorRBG {
    type Output = ColorRBG;

    fn add(self, other: ColorRBG) -> Self::Output {
        ColorRBG {
            r: (self.r + other.r).min(1.0),
            g: (self.g + other.g).min(1.0),
            b: (self.b + other.b).min(1.0),
        }
    }
}

impl Mul<ColorRBG> for ColorRBG {
    type Output = ColorRBG;

    fn mul(self, other: ColorRBG) -> Self::Output {
        ColorRBG {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<ColorRBG> for f64 {
    type Output = ColorRBG;

    fn mul(self, other: ColorRBG) -> Self::Output {
        ColorRBG {
            r: self * other.r,
            g: self * other.g,
            b: self * other.b,
        }
    }
}
