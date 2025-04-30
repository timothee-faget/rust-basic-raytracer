use std::ops::{Add, Mul};

/// RGB Color implementation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorRBG {
    r: f64,
    g: f64,
    b: f64,
}

impl ColorRBG {
    /// RGB Color constructor
    pub fn new(r: f64, g: f64, b: f64) -> ColorRBG {
        ColorRBG { r, g, b }
    }

    /// Get RGB value (0.0 to 1.0)
    #[inline]
    pub fn get_value(&self) -> (f64, f64, f64) {
        (self.r, self.g, self.b)
    }

    /// Get RGB int value (0 to 255)
    #[inline]
    pub fn rgb(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }

    /// Get RGB max value (0.0 to 1.0)
    #[inline]
    pub fn max_component(&self) -> f64 {
        self.r.max(self.g.max(self.b))
    }

    /// RBG BLACK
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    /// RBG WHITE
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    /// RBG RED
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };

    /// RBG GREEN
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };

    /// RBG BLUE
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };

    /// RBG YELLOW
    pub const YELLOW: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 0.0,
    };

    /// RBG PINK
    pub const PINK: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 1.0,
    };

    /// RBG TURQUOISE
    pub const TURQUOISE: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 1.0,
    };

    /// RBG ORANGE
    pub const ORANGE: Self = Self {
        r: 1.0,
        g: 0.5,
        b: 0.0,
    };

    /// RBG LIGHT_BLUE
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

/// RGB Color with overflow implementation
#[derive(Clone, Copy, Debug)]
pub struct ColorRBGOF {
    r: f64,
    g: f64,
    b: f64,
}

impl ColorRBGOF {
    /// RGBOF BLACK
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    /// Converts to ColorRBG
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

/// Lerp 2 Colors
#[inline]
pub fn lerp_color(color_1: ColorRBG, color_2: ColorRBG, t: f64) -> ColorRBG {
    ColorRBG {
        r: color_1.r + (color_2.r - color_1.r) * t,
        g: color_1.g + (color_2.g - color_1.g) * t,
        b: color_1.b + (color_2.b - color_1.b) * t,
    }
}

#[cfg(test)]
mod tests_colors {
    use super::{lerp_color, ColorRBG, ColorRBGOF};

    #[test]
    fn lerp() {
        let color_1 = ColorRBG::BLACK;
        let color_2 = ColorRBG::WHITE;

        assert_eq!(
            lerp_color(color_1, color_2, 0.5),
            ColorRBG::new(0.5, 0.5, 0.5)
        );
    }

    #[test]
    fn rgbof_conversion() {
        let color = ColorRBGOF {
            r: 2.0,
            g: 2.0,
            b: 2.0,
        };

        assert_eq!(color.to_rgb(), ColorRBG::WHITE);
    }
}
