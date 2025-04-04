use std::ops::{Add, Mul, Sub};

// Vectors

#[derive(Debug, Copy, Clone)]
pub struct Vect3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vect3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vect3 { x, y, z }
    }

    pub fn prod(self, other: Self) -> Self {
        Vect3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vect3 {
        self * (1.0 / self.norm())
    }

    // Defaults

    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const UP: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const RIGHT: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const FORWARD: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    pub const DOWN: Self = Self {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };
    pub const LEFT: Self = Self {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const BACKWARD: Self = Self {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
}

impl PartialEq for Vect3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }
}

impl Add for Vect3 {
    type Output = Vect3;

    fn add(self, other: Vect3) -> Vect3 {
        Vect3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vect3 {
    type Output = Vect3;

    fn sub(self, other: Vect3) -> Vect3 {
        Vect3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vect3 {
    type Output = Vect3;

    fn mul(self, scalar: f64) -> Vect3 {
        Vect3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vect3> for f64 {
    type Output = Vect3;

    fn mul(self, vect: Vect3) -> Vect3 {
        Vect3 {
            x: self * vect.x,
            y: self * vect.y,
            z: self * vect.z,
        }
    }
}

impl Mul<f64> for &Vect3 {
    type Output = Vect3;

    fn mul(self, scalar: f64) -> Vect3 {
        Vect3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<&Vect3> for f64 {
    type Output = Vect3;

    fn mul(self, vect: &Vect3) -> Vect3 {
        Vect3 {
            x: self * vect.x,
            y: self * vect.y,
            z: self * vect.z,
        }
    }
}

impl Mul for Vect3 {
    type Output = f64;

    fn mul(self, other: Vect3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

// Position

pub struct Transform {
    position: Vect3,
    orientation: (Vect3, Vect3, Vect3),
}

impl Transform {
    pub fn new(position: Vect3) -> Transform {
        Transform {
            position,
            orientation: (Vect3::RIGHT, Vect3::UP, Vect3::FORWARD),
        }
    }

    pub fn get_right(&self) -> Vect3 {
        self.orientation.0
    }

    pub fn get_up(&self) -> Vect3 {
        self.orientation.1
    }

    pub fn get_front(&self) -> Vect3 {
        self.orientation.2
    }

    pub fn move_x(mut self, value: f64) {
        self.position.x += value;
    }

    pub fn move_y(mut self, value: f64) {
        self.position.y += value;
    }

    pub fn move_z(mut self, value: f64) {
        self.position.z += value;
    }

    pub fn get_pos(&self) -> Vect3 {
        self.position
    }
}

#[cfg(test)]
mod tests_vectors {
    use super::Vect3;

    #[test]
    fn add() {
        let vect3_1 = Vect3::new(1.0, 4.0, -1.0);
        let vect3_2 = Vect3::new(2.0, 1.0, 3.0);

        let vect3_result = vect3_1 + vect3_2;

        assert_eq!(vect3_result.x, 3.0);
        assert_eq!(vect3_result.y, 5.0);
        assert_eq!(vect3_result.z, 2.0);
    }

    #[test]
    fn sub() {
        let vect3_1 = Vect3::new(1.0, 4.0, -1.0);
        let vect3_2 = Vect3::new(2.0, 1.0, 3.0);

        let vect3_result = vect3_1 - vect3_2;

        assert_eq!(vect3_result.x, -1.0);
        assert_eq!(vect3_result.y, 3.0);
        assert_eq!(vect3_result.z, -4.0);
    }

    #[test]
    fn mul() {
        let vect3_1 = Vect3::new(2.0, 0.0, 2.0);
        let vect3_2 = Vect3::new(0.0, 1.0, 0.0);
        let vect3_3 = Vect3::new(3.0, 1.0, 1.0);

        let vect3_result_1 = vect3_1 * vect3_2;
        let vect3_result_2 = vect3_1 * vect3_3;

        assert_eq!(vect3_result_1, 0.0);
        assert_eq!(vect3_result_2, 8.0);
    }

    #[test]
    fn scalar_mul() {
        let vect3 = Vect3::new(1.0, 4.0, -1.0);
        let scalar = 3.5;

        let vect3_result_1 = vect3 * scalar;
        let vect3_result_2 = scalar * vect3;

        assert_eq!(vect3_result_1.x, 3.5);
        assert_eq!(vect3_result_1.y, 14.0);
        assert_eq!(vect3_result_1.z, -3.5);

        assert_eq!(vect3_result_2.x, 3.5);
        assert_eq!(vect3_result_2.y, 14.0);
        assert_eq!(vect3_result_2.z, -3.5);
    }

    #[test]
    fn prod() {
        assert_eq!(Vect3::RIGHT.prod(Vect3::UP), Vect3::FORWARD);
        assert_eq!(
            Vect3::new(5.0, 3.0, -2.0).prod(Vect3::new(-2.0, 4.0, 1.0)),
            Vect3::new(11.0, -1.0, 26.0)
        );
    }

    #[test]
    fn norm() {
        assert_eq!(Vect3::RIGHT.norm(), 1.0);
        assert_eq!(Vect3::new(1.0, 1.0, 1.0).norm(), (3.0_f64).sqrt());
    }

    #[test]
    fn normalize() {
        assert_eq!(Vect3::RIGHT.normalize(), Vect3::RIGHT);
        assert_eq!(Vect3::new(5.0, 0.0, 0.0).normalize(), Vect3::RIGHT)
    }
}
