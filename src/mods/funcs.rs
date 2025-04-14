use super::position::Vect3;

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

pub fn det_from_3_vects(vect_1: Vect3, vect_2: Vect3, vect_3: Vect3) -> f64 {
    vect_1 * (vect_2.prod(vect_3))
}

#[cfg(test)]
mod tests_funcs {
    use crate::mods::position::Vect3;

    use super::det_from_3_vects;

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
}
