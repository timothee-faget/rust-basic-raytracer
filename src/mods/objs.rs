use std::cmp::Ordering;

use super::{
    color::ColorRBG,
    funcs::solve_quadratic,
    position::{Quat, Transform, Vect3},
    render::Ray,
};

// Intersection stuff

#[derive(Debug)]
pub struct Intersection {
    pub distance: f64,
    pub color: ColorRBG,
    pub point: Vect3,
    pub normal: Vect3,
}

impl Intersection {
    pub fn new(distance: f64, color: ColorRBG, point: Vect3, normal: Vect3) -> Intersection {
        Intersection {
            distance,
            color,
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

// Objects stuff

pub trait Object3D {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn get_color(&self) -> (f64, f64, f64);
}

pub struct Sphere {
    transform: Transform,
    radius: f64,
    color: ColorRBG,
}

impl Sphere {
    pub fn new(position: Vect3, radius: f64, color: ColorRBG) -> Sphere {
        Sphere {
            transform: Transform::new(position, Quat::identity()),
            radius,
            color,
        }
    }
}

impl Object3D for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.direction * ray.direction;
        let b = 2.0 * (ray.direction * (ray.start - self.transform.get_pos()));
        let c = (ray.start - self.transform.get_pos()) * (ray.start - self.transform.get_pos())
            - self.radius * self.radius;

        let (t1, t2) = solve_quadratic(a, b, c)?;

        let _distance = match (t1 >= 0.0, t2 >= 0.0) {
            (true, true) => t1.min(t2),
            (true, false) => t1,
            (false, true) => t2,
            (false, false) => return None,
        };

        let distance = t1.min(t2);

        let point = ray.start + distance * ray.direction;
        let normal = (point - self.transform.get_pos()).normalize();
        Some(Intersection::new(distance, self.color, point, normal))
    }

    fn get_color(&self) -> (f64, f64, f64) {
        self.color.get_value()
    }
}
