use std::f64::consts::PI;

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let delta = b * b - 4.0 * a * c;
    if delta < 0.0 {
        None
    } else {
        let delta_sqrt = delta.sqrt();
        let mut t1 = (-b + delta_sqrt) / (2.0 * a);
        let mut t2 = (-b - delta_sqrt) / (2.0 * a);
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }
        Some((t1, t2))
    }
}

// Angle Stuff

pub struct Angle {
    value: f64,
}

impl Angle {
    pub fn new(value: f64) -> Angle {
        Angle { value }
    }

    pub fn from_deg(value_deg: f64) -> Angle {
        Angle {
            value: value_deg * PI / 180.0,
        }
    }

    pub fn get(&self) -> f64 {
        self.value
    }

    pub fn cos(&self) -> f64 {
        self.value.cos()
    }

    pub fn sin(&self) -> f64 {
        self.value.sin()
    }

    pub fn tan(&self) -> f64 {
        self.value.tan()
    }
}
