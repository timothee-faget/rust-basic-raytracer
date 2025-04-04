use std::cmp::Ordering;

use super::{
    camera::Ray,
    color::ColorRBG,
    position::{Quat, Transform, Vect3},
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

        let delta = b * b - 4.0 * a * c;

        if delta < 0.0 {
            None
        } else if delta == 0.0 {
            let distance = -b / (2.0 * a);
            let point = ray.start + distance * ray.direction;
            let normal = (point - self.transform.get_pos()).normalize();
            Some(Intersection::new(distance, self.color, point, normal))
        } else {
            // TODO: Y'a probablement moyen de faire mieux ici
            let distance = [(-b + delta.sqrt()) / 2.0 * a, (-b - delta.sqrt()) / 2.0 * a];
            if distance[0] < distance[1] {
                let point = ray.start + distance[0] * ray.direction;
                let normal = (point - self.transform.get_pos()).normalize();
                Some(Intersection::new(distance[0], self.color, point, normal))
            } else {
                let point = ray.start + distance[1] * ray.direction;
                let normal = (point - self.transform.get_pos()).normalize();
                Some(Intersection::new(distance[1], self.color, point, normal))
            }
        }
    }

    fn get_color(&self) -> (f64, f64, f64) {
        self.color.get_value()
    }
}
