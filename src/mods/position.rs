use core::f64;
use core::f64::consts::PI;
use std::ops::{Add, Div, Mul, Sub};

// Vectors

#[derive(Debug, Copy, Clone)]
pub struct Vect3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vect3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vect3 { x, y, z }
    }

    #[inline]
    pub fn prod(self, other: Self) -> Self {
        Vect3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline]
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn normalize(&self) -> Vect3 {
        self * (1.0 / self.norm())
    }

    #[inline]
    pub fn dot(&self, other: &Vect3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
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

    #[inline]
    fn add(self, other: Vect3) -> Vect3 {
        Vect3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Add<Vect3> for &'a Vect3 {
    type Output = Vect3;

    #[inline]
    fn add(self, other: Vect3) -> Vect3 {
        Vect3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl<'b> Add<&'b Vect3> for Vect3 {
    type Output = Vect3;

    #[inline]
    fn add(self, other: &'b Vect3) -> Vect3 {
        Vect3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl<'a, 'b> Add<&'b Vect3> for &'a Vect3 {
    type Output = Vect3;

    #[inline]
    fn add(self, other: &'b Vect3) -> Vect3 {
        Vect3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vect3 {
    type Output = Vect3;

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn mul(self, other: Vect3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[inline]
pub fn lerp(vect_1: Vect3, vect_2: Vect3, t: f64) -> Vect3 {
    vect_1 + t * (vect_2 - vect_1)
}

// Quaternion stuff

#[derive(Debug, Clone, Copy)]
pub struct Quat {
    w: f64,
    v: Vect3,
}

impl Quat {
    pub fn new(w: f64, v: Vect3) -> Quat {
        Quat { w, v }
    }

    #[inline]
    pub fn identity() -> Self {
        Quat {
            w: 1.0,
            v: Vect3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn from_axis_angle(axis: Vect3, angle: f64) -> Self {
        let half_angle = Angle::new(angle / 2.0);
        let half_angle_sin = half_angle.sin();
        Quat {
            w: half_angle.cos(),
            v: half_angle_sin * axis,
        }
    }

    pub fn from_axis_angle_deg(axis: Vect3, angle: f64) -> Self {
        let half_angle = Angle::from_deg(angle / 2.0);
        let half_angle_sin = half_angle.sin();
        Quat {
            w: half_angle.cos(),
            v: half_angle_sin * axis,
        }
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let norm =
            (self.w * self.w + self.v.x * self.v.x + self.v.y * self.v.y + self.v.z * self.v.z)
                .sqrt();
        if norm == 0.0 {
            self
        } else {
            Quat {
                w: self.w / norm,
                v: self.v * (1.0 / norm),
            }
        }
    }

    #[inline]
    pub fn conjugate(self) -> Self {
        Quat {
            w: self.w,
            v: -1.0 * self.v,
        }
    }

    #[inline]
    pub fn rotate(self, v: Vect3) -> Vect3 {
        let q_v = Quat { w: 0.0, v };
        let q_inv = self.conjugate();
        let result = self * q_v * q_inv;

        Vect3::new(result.v.x, result.v.y, result.v.z)
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            w: self.w * rhs.w - self.v.x * rhs.v.x - self.v.y * rhs.v.y - self.v.z * rhs.v.z,
            v: Vect3 {
                x: self.w * rhs.v.x + self.v.x * rhs.w + self.v.y * rhs.v.z - self.v.z * rhs.v.y,
                y: self.w * rhs.v.y + self.v.y * rhs.w + self.v.z * rhs.v.x - self.v.x * rhs.v.z,
                z: self.w * rhs.v.z + self.v.z * rhs.w + self.v.x * rhs.v.y - self.v.y * rhs.v.x,
            },
        }
    }
}

// Transform stuff

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    position: Vect3,
    rotation: Quat,
}

impl Transform {
    pub fn new(position: Vect3, rotation: Quat) -> Transform {
        Transform { position, rotation }
    }

    // Setters

    pub fn rotate(&mut self, rot: Quat) {
        self.rotation = rot * self.rotation;
    }

    pub fn rotate_around_x_axis_deg(&mut self, angle: f64) {
        let rot = Quat::from_axis_angle_deg(self.get_x_axis(), angle);
        self.rotation = rot * self.rotation;
    }

    pub fn rotate_around_y_axis_deg(&mut self, angle: f64) {
        let rot = Quat::from_axis_angle_deg(self.get_y_axis(), angle);
        self.rotation = rot * self.rotation;
    }

    pub fn rotate_around_z_axis_deg(&mut self, angle: f64) {
        let rot = Quat::from_axis_angle_deg(self.get_z_axis(), angle);
        self.rotation = rot * self.rotation;
    }
    // Getters

    #[inline]
    pub fn get_pos(&self) -> Vect3 {
        self.position
    }

    pub fn get_x_axis(&self) -> Vect3 {
        self.rotation.rotate(Vect3::RIGHT)
    }

    pub fn get_y_axis(&self) -> Vect3 {
        self.rotation.rotate(Vect3::UP)
    }

    pub fn get_z_axis(&self) -> Vect3 {
        self.rotation.rotate(Vect3::FORWARD)
    }
}

// Angle Stuff

#[derive(Clone, Copy)]
pub struct Angle {
    value: f64,
}

impl Angle {
    pub fn new(value: f64) -> Angle {
        Angle { value }
    }

    #[inline]
    pub fn from_deg(value_deg: f64) -> Angle {
        Angle {
            value: value_deg * PI / 180.0,
        }
    }

    pub fn get(&self) -> f64 {
        self.value
    }

    #[inline]
    pub fn cos(&self) -> f64 {
        self.value.cos()
    }

    #[inline]
    pub fn sin(&self) -> f64 {
        self.value.sin()
    }

    #[inline]
    pub fn tan(&self) -> f64 {
        self.value.tan()
    }
}

impl Div<f64> for Angle {
    type Output = Angle;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            value: self.value / rhs,
        }
    }
}

impl Mul<f64> for Angle {
    type Output = Angle;

    fn mul(self, rhs: f64) -> Angle {
        Self {
            value: self.value * rhs,
        }
    }
}

impl Mul<Angle> for f64 {
    type Output = Angle;

    fn mul(self, rhs: Angle) -> Angle {
        Angle {
            value: self * rhs.value,
        }
    }
}

// Tests

#[cfg(test)]
mod tests_quaternions {
    use super::{Quat, Transform, Vect3};
    use approx::assert_abs_diff_eq;
    use approx::{AbsDiffEq, RelativeEq};

    // Implémentation pour les tests (à bouger à terme dans les modules de test?)
    impl AbsDiffEq for Vect3 {
        type Epsilon = f64;

        fn default_epsilon() -> Self::Epsilon {
            1e-10
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            f64::abs_diff_eq(&self.x, &other.x, epsilon)
                && f64::abs_diff_eq(&self.y, &other.y, epsilon)
                && f64::abs_diff_eq(&self.z, &other.z, epsilon)
        }
    }

    // Implémentation pour les tests (à bouger à terme dans les modules de test?)
    impl RelativeEq for Vect3 {
        fn default_max_relative() -> Self::Epsilon {
            1e10
        }

        fn relative_eq(
            &self,
            other: &Self,
            epsilon: Self::Epsilon,
            max_relative: Self::Epsilon,
        ) -> bool {
            f64::relative_eq(&self.x, &other.x, epsilon, max_relative)
                && f64::relative_eq(&self.y, &other.y, epsilon, max_relative)
                && f64::relative_eq(&self.z, &other.z, epsilon, max_relative)
        }
    }

    #[test]
    fn rotation_90() {
        let transform = Transform::new(Vect3::ZERO, Quat::from_axis_angle_deg(Vect3::UP, 90.0));

        assert_abs_diff_eq!(transform.get_x_axis(), Vect3::new(0.0, 0.0, -1.0));
        assert_abs_diff_eq!(transform.get_y_axis(), Vect3::new(0.0, 1.0, 0.0));
        assert_abs_diff_eq!(transform.get_z_axis(), Vect3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotation_180() {
        let transform = Transform::new(Vect3::ZERO, Quat::from_axis_angle_deg(Vect3::RIGHT, 180.0));

        assert_abs_diff_eq!(transform.get_x_axis(), Vect3::new(1.0, 0.0, 0.0));
        assert_abs_diff_eq!(transform.get_y_axis(), Vect3::new(0.0, -1.0, 0.0));
        assert_abs_diff_eq!(transform.get_z_axis(), Vect3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn rotation_around_x_axis() {
        let mut transform = Transform::new(Vect3::ZERO, Quat::identity());

        transform.rotate_around_x_axis_deg(90.0);

        assert_abs_diff_eq!(transform.get_x_axis(), Vect3::RIGHT);
        assert_abs_diff_eq!(transform.get_y_axis(), Vect3::FORWARD);
        assert_abs_diff_eq!(transform.get_z_axis(), Vect3::DOWN);

        transform.rotate_around_x_axis_deg(-90.0);

        assert_abs_diff_eq!(transform.get_x_axis(), Vect3::RIGHT);
        assert_abs_diff_eq!(transform.get_y_axis(), Vect3::UP);
        assert_abs_diff_eq!(transform.get_z_axis(), Vect3::FORWARD);
    }

    #[test]
    fn rotation_around_y_axis() {
        let mut transform = Transform::new(Vect3::ZERO, Quat::identity());

        transform.rotate_around_y_axis_deg(90.0);

        assert_abs_diff_eq!(transform.get_x_axis(), Vect3::BACKWARD);
        assert_abs_diff_eq!(transform.get_y_axis(), Vect3::UP);
        assert_abs_diff_eq!(transform.get_z_axis(), Vect3::RIGHT);

        transform.rotate_around_y_axis_deg(-90.0);

        assert_abs_diff_eq!(transform.get_x_axis(), Vect3::RIGHT);
        assert_abs_diff_eq!(transform.get_y_axis(), Vect3::UP);
        assert_abs_diff_eq!(transform.get_z_axis(), Vect3::FORWARD);
    }

    #[test]
    fn rotation_around_z_axis() {
        let mut transform = Transform::new(Vect3::ZERO, Quat::identity());

        transform.rotate_around_z_axis_deg(90.0);

        assert_abs_diff_eq!(transform.get_x_axis(), Vect3::UP);
        assert_abs_diff_eq!(transform.get_y_axis(), Vect3::LEFT);
        assert_abs_diff_eq!(transform.get_z_axis(), Vect3::FORWARD);

        transform.rotate_around_z_axis_deg(-90.0);

        assert_abs_diff_eq!(transform.get_x_axis(), Vect3::RIGHT);
        assert_abs_diff_eq!(transform.get_y_axis(), Vect3::UP);
        assert_abs_diff_eq!(transform.get_z_axis(), Vect3::FORWARD);
    }

    #[test]
    fn rotate_around_all_axis() {
        let mut transform = Transform::new(Vect3::ZERO, Quat::identity());

        transform.rotate_around_x_axis_deg(90.0);
        transform.rotate_around_y_axis_deg(90.0);
        transform.rotate_around_z_axis_deg(90.0);

        assert_abs_diff_eq!(transform.get_x_axis(), Vect3::FORWARD);
        assert_abs_diff_eq!(transform.get_y_axis(), Vect3::DOWN);
        assert_abs_diff_eq!(transform.get_z_axis(), Vect3::RIGHT);
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
