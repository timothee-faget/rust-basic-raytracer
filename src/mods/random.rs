use super::position::Vect3;

/// LCG Random Generator implementation
#[derive(Debug, Clone, Copy)]
pub struct LCG {
    state: u64,
}

impl LCG {
    pub fn new(seed: u64) -> Self {
        LCG { state: seed }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.state = seed;
    }

    /// Get next state
    #[inline]
    fn next(&mut self) -> u64 {
        // Constants choisies pour de bons résultats
        const A: u64 = 6364136223846793005;
        const C: u64 = 1442695040888963407;
        self.state = A.wrapping_mul(self.state).wrapping_add(C);
        self.state
    }

    /// Get next random f64
    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        (self.next() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// Get next random Vect3
    #[inline]
    pub fn next_vect3(&mut self) -> Vect3 {
        Vect3::new(self.next_f64(), self.next_f64(), self.next_f64())
    }

    /// Get next random Vect3 with normal
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
    use super::LCG;
    use crate::mods::position::Vect3;

    #[test]
    fn test_random_normal_vect() {
        let normal = Vect3::UP;
        let mut rand = LCG::new(213456789);

        for _ in 0..100 {
            assert!(rand.next_normal_vect3(normal).y() >= 0.0);
        }
    }
}
