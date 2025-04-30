use super::position::Vect3;

/// Solve a quadratic equation from its coefficients
#[inline]
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

/// Gets the determinent of a 3x3 array
#[inline]
pub fn det_from_3_vects(vect_1: Vect3, vect_2: Vect3, vect_3: Vect3) -> f64 {
    vect_1 * (vect_2.prod(vect_3))
}

/// Reflect a 3D vcetor with a normal
#[inline]
pub fn reflect(in_vect: Vect3, normal_vect: Vect3) -> Vect3 {
    in_vect - 2.0 * normal_vect * (in_vect * normal_vect)
}

/// Converts secs from f64 to [h:m:s] format
pub fn s_to_hms(secs: f64) -> String {
    let h = (secs / 3600.0).trunc();
    let m = ((secs - h * 3600.0) / 60.0).trunc();
    let s = ((secs - h * 3600.0) - m * 60.0).trunc();

    format!("[{}:{}:{}]", h, m, s)
}

#[cfg(test)]
mod tests_funcs {
    use approx::assert_abs_diff_eq;

    use crate::mods::position::Vect3;

    use super::det_from_3_vects;
    use super::reflect;
    use super::s_to_hms;

    #[test]
    fn test_det() {
        let vect_1 = Vect3::new(-1.0, 1.0, -2.0);
        let vect_2 = Vect3::new(2.0, 2.0, 8.0);
        let vect_3 = Vect3::new(5.0, 3.0, 10.0);

        assert_eq!(det_from_3_vects(vect_1, vect_2, vect_3), 32.0);

        let vect_1 = Vect3::new(1.0, 0.0, 3.0);
        let vect_2 = Vect3::new(6.0, 4.0, 2.0);
        let vect_3 = Vect3::new(0.0, -2.0, 5.0);

        assert_eq!(det_from_3_vects(vect_1, vect_2, vect_3), -12.0)
    }

    #[test]
    fn test_reflect() {
        let vect = Vect3::new(0.707, -0.707, 0.0);
        assert_abs_diff_eq!(reflect(vect, Vect3::UP), Vect3::new(0.707, 0.707, 0.0));
    }

    #[test]
    fn secs_to_hms() {
        assert_eq!(s_to_hms(112.0), String::from("[0:1:52]"));
        assert_eq!(s_to_hms(0.0), String::from("[0:0:0]"));
        assert_eq!(s_to_hms(120.0), String::from("[0:2:0]"));
        assert_eq!(s_to_hms(3674.0), String::from("[1:1:14]"));
    }
}
