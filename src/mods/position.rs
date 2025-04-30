use core::f64;
use core::f64::consts::PI;
use std::ops::{Add, Div, Mul, Sub};
use std::simd::{cmp::SimdPartialEq, f64x4};

/// 3D vector implementation using SIMD
#[derive(Debug, Copy, Clone)]
pub struct Vect3 {
    // Utilisation de f64x4 pour stocker [x, y, z, 0]
    simd: f64x4,
}

impl Vect3 {
    /// New vector creator
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            simd: f64x4::from_array([x, y, z, 0.0]),
        }
    }

    /// Accesseur pour x
    #[inline]
    pub fn x(&self) -> f64 {
        self.simd[0]
    }

    /// Accesseur pour y
    #[inline]
    pub fn y(&self) -> f64 {
        self.simd[1]
    }

    /// Accesseur pour z
    #[inline]
    pub fn z(&self) -> f64 {
        self.simd[2]
    }

    /// Creates vector from array
    pub fn from_arr(arr: [f64; 3]) -> Self {
        Self {
            simd: f64x4::from_array([arr[0], arr[1], arr[2], 0.0]),
        }
    }

    /// Cross product beteen 2 vectors
    #[inline]
    pub fn prod(self, other: Self) -> Self {
        // Effectue le produit vectoriel manuellement
        // x = self.y * other.z - self.z * other.y
        // y = self.z * other.x - self.x * other.z
        // z = self.x * other.y - self.y * other.x
        let a_yzx = f64x4::from_array([self.y(), self.z(), self.x(), 0.0]);
        let b_zxy = f64x4::from_array([other.z(), other.x(), other.y(), 0.0]);

        let a_zxy = f64x4::from_array([self.z(), self.x(), self.y(), 0.0]);
        let b_yzx = f64x4::from_array([other.y(), other.z(), other.x(), 0.0]);

        let product = a_yzx * b_zxy - a_zxy * b_yzx;

        Vect3 {
            simd: f64x4::from_array([product[0], product[1], product[2], 0.0]),
        }
    }

    /// Length of a vector
    #[inline]
    pub fn norm(&self) -> f64 {
        // Calcule x² + y² + z² puis prend la racine carrée
        let squared = self.simd * self.simd;
        let sum = squared[0] + squared[1] + squared[2];
        sum.sqrt()
    }

    /// Normalize the vector
    #[inline]
    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        if norm == 0.0 {
            return Self::ZERO;
        }

        let inv_norm = 1.0 / norm;
        Self {
            simd: self.simd * f64x4::splat(inv_norm),
        }
    }

    /// Dot product of 2 vectors
    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        let product = self.simd * other.simd;
        product[0] + product[1] + product[2]
    }

    /// 3D vector to f64 array
    #[inline]
    pub fn to_arr(&self) -> [f64; 3] {
        [self.x(), self.y(), self.z()]
    }

    /// Default ZERO vector
    pub const ZERO: Self = Self {
        simd: f64x4::from_array([0.0, 0.0, 0.0, 0.0]),
    };

    /// Default UP vector
    pub const UP: Self = Self {
        simd: f64x4::from_array([0.0, 1.0, 0.0, 0.0]),
    };

    /// Default RIGHT vector
    pub const RIGHT: Self = Self {
        simd: f64x4::from_array([1.0, 0.0, 0.0, 0.0]),
    };

    /// Default FORWARD vector
    pub const FORWARD: Self = Self {
        simd: f64x4::from_array([0.0, 0.0, 1.0, 0.0]),
    };

    /// Default DOWN vector
    pub const DOWN: Self = Self {
        simd: f64x4::from_array([0.0, -1.0, 0.0, 0.0]),
    };

    /// Default LEFT vector
    pub const LEFT: Self = Self {
        simd: f64x4::from_array([-1.0, 0.0, 0.0, 0.0]),
    };

    /// Default BACKWARD vector
    pub const BACKWARD: Self = Self {
        simd: f64x4::from_array([0.0, 0.0, -1.0, 0.0]),
    };
}

impl PartialEq for Vect3 {
    fn eq(&self, other: &Self) -> bool {
        let mask = self.simd.simd_eq(other.simd);
        // Vérifier que les 3 premières composantes sont égales
        let mask_bits = mask.to_bitmask();
        (mask_bits & 0b0111) == 0b0111
    }
}

impl Add for Vect3 {
    type Output = Vect3;

    #[inline]
    fn add(self, other: Vect3) -> Vect3 {
        Vect3 {
            simd: self.simd + other.simd,
        }
    }
}

impl<'a> Add<Vect3> for &'a Vect3 {
    type Output = Vect3;

    #[inline]
    fn add(self, other: Vect3) -> Vect3 {
        Vect3 {
            simd: self.simd + other.simd,
        }
    }
}

impl<'b> Add<&'b Vect3> for Vect3 {
    type Output = Vect3;

    #[inline]
    fn add(self, other: &'b Vect3) -> Vect3 {
        Vect3 {
            simd: self.simd + other.simd,
        }
    }
}

impl<'a, 'b> Add<&'b Vect3> for &'a Vect3 {
    type Output = Vect3;

    #[inline]
    fn add(self, other: &'b Vect3) -> Vect3 {
        Vect3 {
            simd: self.simd + other.simd,
        }
    }
}

impl Sub for Vect3 {
    type Output = Vect3;

    #[inline]
    fn sub(self, other: Vect3) -> Vect3 {
        Vect3 {
            simd: self.simd - other.simd,
        }
    }
}

impl Mul<f64> for Vect3 {
    type Output = Vect3;

    #[inline]
    fn mul(self, scalar: f64) -> Vect3 {
        Vect3 {
            simd: self.simd * f64x4::splat(scalar),
        }
    }
}

impl Mul<Vect3> for f64 {
    type Output = Vect3;

    #[inline]
    fn mul(self, vect: Vect3) -> Vect3 {
        Vect3 {
            simd: f64x4::splat(self) * vect.simd,
        }
    }
}

impl Mul<f64> for &Vect3 {
    type Output = Vect3;

    fn mul(self, scalar: f64) -> Vect3 {
        Vect3 {
            simd: self.simd * f64x4::splat(scalar),
        }
    }
}

impl Mul<&Vect3> for f64 {
    type Output = Vect3;

    #[inline]
    fn mul(self, vect: &Vect3) -> Vect3 {
        Vect3 {
            simd: f64x4::splat(self) * vect.simd,
        }
    }
}

impl Mul for Vect3 {
    type Output = f64;

    #[inline]
    fn mul(self, other: Vect3) -> f64 {
        self.dot(&other)
    }
}

/// 3D vector lerper
#[inline]
pub fn lerp(vect_1: Vect3, vect_2: Vect3, t: f64) -> Vect3 {
    vect_1 + t * (vect_2 - vect_1)
}

/// Quaternion implementation with SIMD
#[derive(Debug, Clone, Copy)]
pub struct Quat {
    // Stocke [x, y, z, w] - remarquez que w est à la fin pour faciliter certaines opérations
    simd: f64x4,
}

impl Quat {
    /// New quaternion constructor
    pub fn new(w: f64, v: Vect3) -> Quat {
        Quat {
            simd: f64x4::from_array([v.x(), v.y(), v.z(), w]),
        }
    }

    /// Récupère la partie réelle (w)
    #[inline]
    pub fn w(&self) -> f64 {
        self.simd[3]
    }

    /// Récupère la partie vectorielle (v)
    #[inline]
    pub fn v(&self) -> Vect3 {
        Vect3::new(self.simd[0], self.simd[1], self.simd[2])
    }

    /// Identity (no rotation no scaling)
    #[inline]
    pub fn identity() -> Self {
        Quat {
            simd: f64x4::from_array([0.0, 0.0, 0.0, 1.0]),
        }
    }

    /// Creates quaternion from axis and angle (rad)
    pub fn from_axis_angle(axis: Vect3, angle: f64) -> Self {
        let half_angle = Angle::new(angle / 2.0);
        let half_angle_sin = half_angle.sin();
        let normalized_axis = axis.normalize();

        Quat {
            simd: f64x4::from_array([
                normalized_axis.x() * half_angle_sin,
                normalized_axis.y() * half_angle_sin,
                normalized_axis.z() * half_angle_sin,
                half_angle.cos(),
            ]),
        }
    }

    /// Creates quaternion from axis and angle (deg)
    pub fn from_axis_angle_deg(axis: Vect3, angle: f64) -> Self {
        let half_angle = Angle::from_deg(angle / 2.0);
        let half_angle_sin = half_angle.sin();
        let normalized_axis = axis.normalize();

        Quat {
            simd: f64x4::from_array([
                normalized_axis.x() * half_angle_sin,
                normalized_axis.y() * half_angle_sin,
                normalized_axis.z() * half_angle_sin,
                half_angle.cos(),
            ]),
        }
    }

    /// Normalizes quaternion
    #[inline]
    pub fn normalize(self) -> Self {
        let squared = self.simd * self.simd;
        let sum = squared[0] + squared[1] + squared[2] + squared[3];

        if sum == 0.0 {
            return self;
        }

        let norm = sum.sqrt();
        let inv_norm = 1.0 / norm;

        Quat {
            simd: self.simd * f64x4::splat(inv_norm),
        }
    }

    /// Conjugates quaternion
    #[inline]
    pub fn conjugate(self) -> Self {
        // Inverse les parties x, y, z mais pas w
        let conj_mask = f64x4::from_array([-1.0, -1.0, -1.0, 1.0]);
        Quat {
            simd: self.simd * conj_mask,
        }
    }

    /// Rotates a 3D vector from self
    #[inline]
    pub fn rotate(self, v: Vect3) -> Vect3 {
        // Convertit le vecteur en quaternion avec w=0
        let q_v = Quat::new(0.0, v);
        let q_inv = self.conjugate();

        // q * q_v * q^-1
        let result = self * q_v * q_inv;

        // Extrait la partie vectorielle du résultat
        result.v()
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // Multiplication quaternion: (a, u) * (b, v) = (ab - u·v, av + bu + u×v)
        let a = self.w();
        let b = rhs.w();
        let u = self.v();
        let v = rhs.v();

        // Partie scalaire: ab - u·v
        let w = a * b - u.dot(&v);

        // Partie vectorielle: av + bu + u×v
        let vec_part = v * a + u * b + u.prod(v);

        Self::new(w, vec_part)
    }
}

/// Transform implementation
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    position: Vect3,
    rotation: Quat,
}

impl Transform {
    /// Transform constructor
    pub fn new(position: Vect3, rotation: Quat) -> Transform {
        Transform { position, rotation }
    }

    /// Adds rotation
    pub fn rotate(&mut self, rot: Quat) {
        self.rotation = rot * self.rotation;
    }

    /// Rotates around X axis from angle (deg)
    pub fn rotate_around_x_axis_deg(&mut self, angle: f64) {
        let rot = Quat::from_axis_angle_deg(self.get_x_axis(), angle);
        self.rotation = rot * self.rotation;
    }

    /// Rotates around Y axis from angle (deg)
    pub fn rotate_around_y_axis_deg(&mut self, angle: f64) {
        let rot = Quat::from_axis_angle_deg(self.get_y_axis(), angle);
        self.rotation = rot * self.rotation;
    }

    /// Rotates around Z axis from angle (deg)
    pub fn rotate_around_z_axis_deg(&mut self, angle: f64) {
        let rot = Quat::from_axis_angle_deg(self.get_z_axis(), angle);
        self.rotation = rot * self.rotation;
    }

    /// Get transform position
    #[inline]
    pub fn get_pos(&self) -> Vect3 {
        self.position
    }

    /// Get transform X axis
    pub fn get_x_axis(&self) -> Vect3 {
        self.rotation.rotate(Vect3::RIGHT)
    }

    /// Get transform Y axis
    pub fn get_y_axis(&self) -> Vect3 {
        self.rotation.rotate(Vect3::UP)
    }

    /// Get transform Z axis
    pub fn get_z_axis(&self) -> Vect3 {
        self.rotation.rotate(Vect3::FORWARD)
    }
}

/// Angle implementation
#[derive(Clone, Copy)]
pub struct Angle {
    value: f64,
}

impl Angle {
    /// New angle constructor
    pub fn new(value: f64) -> Angle {
        Angle { value }
    }

    /// Creates angle from deg value
    #[inline]
    pub fn from_deg(value_deg: f64) -> Angle {
        Angle {
            value: value_deg * PI / 180.0,
        }
    }

    /// Get value
    pub fn get(&self) -> f64 {
        self.value
    }

    /// Get angle cos
    #[inline]
    pub fn cos(&self) -> f64 {
        self.value.cos()
    }

    /// Get angle sin
    #[inline]
    pub fn sin(&self) -> f64 {
        self.value.sin()
    }

    /// Get angle tan
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
            f64::abs_diff_eq(&self.x(), &other.x(), epsilon)
                && f64::abs_diff_eq(&self.y(), &other.y(), epsilon)
                && f64::abs_diff_eq(&self.z(), &other.z(), epsilon)
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
            f64::relative_eq(&self.x(), &other.x(), epsilon, max_relative)
                && f64::relative_eq(&self.y(), &other.y(), epsilon, max_relative)
                && f64::relative_eq(&self.z(), &other.z(), epsilon, max_relative)
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

        assert_eq!(vect3_result.x(), 3.0);
        assert_eq!(vect3_result.y(), 5.0);
        assert_eq!(vect3_result.z(), 2.0);
    }

    #[test]
    fn sub() {
        let vect3_1 = Vect3::new(1.0, 4.0, -1.0);
        let vect3_2 = Vect3::new(2.0, 1.0, 3.0);

        let vect3_result = vect3_1 - vect3_2;

        assert_eq!(vect3_result.x(), -1.0);
        assert_eq!(vect3_result.y(), 3.0);
        assert_eq!(vect3_result.z(), -4.0);
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

        assert_eq!(vect3_result_1.x(), 3.5);
        assert_eq!(vect3_result_1.y(), 14.0);
        assert_eq!(vect3_result_1.z(), -3.5);

        assert_eq!(vect3_result_2.x(), 3.5);
        assert_eq!(vect3_result_2.y(), 14.0);
        assert_eq!(vect3_result_2.z(), -3.5);
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
