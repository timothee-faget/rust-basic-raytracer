use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug)]
pub struct ColorRBGOF {
    r: f64,
    g: f64,
    b: f64,
}

impl ColorRBGOF {
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    #[inline]
    pub fn to_rgb(&self) -> ColorRBG {
        ColorRBG {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
        }
    }
}

impl Add<ColorRBG> for ColorRBGOF {
    type Output = ColorRBGOF;

    #[inline]
    fn add(self, other: ColorRBG) -> Self::Output {
        ColorRBGOF {
            r: (self.r + other.r),
            g: (self.g + other.g),
            b: (self.b + other.b),
        }
    }
}

impl Mul<ColorRBGOF> for f64 {
    type Output = ColorRBGOF;

    #[inline]
    fn mul(self, other: ColorRBGOF) -> Self::Output {
        ColorRBGOF {
            r: (self * other.r),
            g: (self * other.g),
            b: (self * other.b),
        }
    }
}

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

    #[inline]
    pub fn get_value(&self) -> (f64, f64, f64) {
        (self.r, self.g, self.b)
    }

    #[inline]
    pub fn rgb(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }

    #[inline]
    pub fn max_component(&self) -> f64 {
        self.r.max(self.g.max(self.b))
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

    #[inline]
    fn add(self, other: ColorRBG) -> Self::Output {
        ColorRBG {
            r: (self.r + other.r).clamp(0.0, 1.0),
            g: (self.g + other.g).clamp(0.0, 1.0),
            b: (self.b + other.b).clamp(0.0, 1.0),
        }
    }
}

impl Mul<ColorRBG> for ColorRBG {
    type Output = ColorRBG;

    #[inline]
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

    #[inline]
    fn mul(self, other: ColorRBG) -> Self::Output {
        ColorRBG {
            r: (self * other.r).clamp(0.0, 1.0),
            g: (self * other.g).clamp(0.0, 1.0),
            b: (self * other.b).clamp(0.0, 1.0),
        }
    }
}

#[inline]
pub fn lerp_color(color_1: ColorRBG, color_2: ColorRBG, t: f64) -> ColorRBG {
    ColorRBG {
        r: color_1.r + (color_2.r - color_1.r) * t,
        g: color_1.g + (color_2.g - color_1.g) * t,
        b: color_1.b + (color_2.b - color_1.b) * t,
    }
}
