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

#[inline]
pub fn reflect(in_vect: Vect3, normal_vect: Vect3) -> Vect3 {
    2.0 * normal_vect * (in_vect * normal_vect) - in_vect
}

// Random Number stuff

#[derive(Debug, Clone, Copy)]
pub struct LCG {
    state: u64,
}

impl LCG {
    pub fn new(seed: u64) -> Self {
        LCG { state: seed }
    }

    #[inline]
    fn next(&mut self) -> u64 {
        // Constants choisies pour de bons résultats
        const A: u64 = 6364136223846793005;
        const C: u64 = 1442695040888963407;
        self.state = A.wrapping_mul(self.state).wrapping_add(C);
        self.state
    }

    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        (self.next() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    #[inline]
    pub fn next_vect3(&mut self) -> Vect3 {
        Vect3::new(self.next_f64(), self.next_f64(), self.next_f64())
    }

    #[inline]
    pub fn next_normal_vect3(&mut self, normal: Vect3) -> Vect3 {
        loop {
            let x = 2.0 * self.next_f64() - 1.0;
            let y = 2.0 * self.next_f64() - 1.0;
            let z = 2.0 * self.next_f64() - 1.0;
            let len_squared = x * x + y * y + z * z;

            if len_squared > 0.0001 && len_squared < 1.0 {
                let vect = Vect3::new(x, y, z).normalize();
                return if vect * normal < 0.0 {
                    -1.0 * vect
                } else {
                    vect
                };
            }
        }
    }
}

#[cfg(test)]
mod tests_funcs {
    use crate::mods::position::Vect3;

    use super::det_from_3_vects;
    use super::LCG;

    #[test]
    fn test_random_normal_vect() {
        let normal = Vect3::UP;
        let mut rand = LCG::new(213456789);

        for _ in 0..100 {
            assert!(rand.next_normal_vect3(normal).y >= 0.0);
        }
    }

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
