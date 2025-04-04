use std::f64::consts::PI;

pub fn is_in_circle(pos: &(i64, i64), r: &i64, x: &i64, y: &i64) -> bool {
    (x - pos.0) * (x - pos.0) + (y - pos.1) * (y - pos.1) < (*r * *r)
}

pub fn gradient_from_center(pos: &(i64, i64), r: &i64, x: &i64, y: &i64) -> u8 {
    let dist = (x - pos.0) * (x - pos.0) + (y - pos.1) * (y - pos.1);
    let color = 255.0 * (dist as f64).sqrt() / *r as f64;
    color as u8
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let delta = b * b - 4.0 * a * c;
    if delta < 0.0 {
        None
    } else {
        let delta_sqrt = delta.sqrt();
        let t1 = (-b + delta_sqrt) / (2.0 * a);
        let t2 = (-b - delta_sqrt) / (2.0 * a);
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
