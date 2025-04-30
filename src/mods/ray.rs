use std::cmp::Ordering;

use super::{material::Material, position::Vect3};

/// Ray implementation
pub struct Ray {
    start: Vect3,
    direction: Vect3,
}

impl Ray {
    /// New Ray constructor
    pub fn new(start: Vect3, direction: Vect3) -> Self {
        Self { start, direction }
    }

    /// Get Ray start
    pub fn get_start(&self) -> Vect3 {
        self.start
    }

    /// Get Ray direction
    pub fn get_dir(&self) -> Vect3 {
        self.direction
    }
}

/// Intersection implementation
#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub distance: f64,
    pub material: Material,
    pub point: Vect3,
    pub normal: Vect3,
}

impl Intersection {
    /// New Intersection constructor
    pub fn new(distance: f64, material: Material, point: Vect3, normal: Vect3) -> Intersection {
        Intersection {
            distance,
            material,
            point,
            normal,
        }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Intersection {}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance < other.distance {
            Ordering::Less
        } else if self.distance == other.distance {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Intersection {
    fn ge(&self, other: &Self) -> bool {
        self.distance >= other.distance
    }

    fn gt(&self, other: &Self) -> bool {
        self.distance > other.distance
    }

    fn le(&self, other: &Self) -> bool {
        self.distance <= other.distance
    }

    fn lt(&self, other: &Self) -> bool {
        self.distance < other.distance
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests_rays {
    use crate::mods::position::Vect3;

    use super::Ray;

    #[test]
    fn ray() {
        let ray = Ray::new(Vect3::ZERO, Vect3::UP);

        assert_eq!(ray.get_start(), Vect3::ZERO);
        assert_eq!(ray.get_dir(), Vect3::UP);
    }
}

#[cfg(test)]
mod tests_intersections {
    use crate::mods::{material::Material, position::Vect3};

    use super::Intersection;

    #[test]
    fn comparison() {
        let inter_1 = Intersection::new(1.0, Material::default(), Vect3::ZERO, Vect3::UP);
        let inter_2 = Intersection::new(2.0, Material::default(), Vect3::ZERO, Vect3::UP);

        assert!(inter_2 > inter_1);
    }
}
