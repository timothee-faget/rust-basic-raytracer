use core::f64;
use std::cmp::Ordering;

use crate::mods::funcs::det_from_3_vects;

use super::{
    //color::ColorRBG,
    funcs::solve_quadratic,
    position::{Quat, Transform, Vect3},
    render::{Material, Ray},
};

// Intersection stuff

#[derive(Debug)]
pub struct Intersection {
    pub distance: f64,
    pub material: Material,
    pub point: Vect3,
    pub normal: Vect3,
}

impl Intersection {
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

// Object3D trait

pub trait Object3D {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn rotate(&mut self, rotation: Quat);
    fn print(&self);
}

// Sphere stuff

pub struct Sphere {
    transform: Transform,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(position: Vect3, radius: f64, material: Material) -> Sphere {
        Sphere {
            transform: Transform::new(position, Quat::identity()),
            radius,
            material,
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

        let distance = match (t1 >= 0.0, t2 >= 0.0) {
            (true, true) => t1.min(t2),
            (true, false) => t1,
            (false, true) => t2,
            (false, false) => return None,
        };

        let point = ray.start + distance * ray.direction;
        let normal = (point - self.transform.get_pos()).normalize();
        Some(Intersection::new(distance, self.material, point, normal))
    }

    fn rotate(&mut self, rotation: Quat) {
        self.transform.rotate(rotation);
    }

    fn print(&self) {
        println!(
            "Sphère de centre {:?} et de rayon {:?}",
            self.transform.get_pos(),
            self.radius
        );
    }
}

// Plane stuff

pub struct Plane {
    point: Vect3,
    normal: Vect3,
    material: Material,
}

impl Plane {
    pub fn new(point: Vect3, normal: Vect3, material: Material) -> Self {
        Self {
            point,
            normal: normal.normalize(), // Au cas ou
            material,
        }
    }
}

impl Object3D for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if (ray.direction * self.normal).abs() <= 1e-5 {
            return None;
        }

        let distance = -((ray.start - self.point) * self.normal) / (ray.direction * self.normal);

        if distance > 0.0 {
            let point = ray.start + distance * ray.direction;
            let normal = self.normal;
            Some(Intersection::new(distance, self.material, point, normal))
        } else {
            None
        }
    }

    fn rotate(&mut self, rotation: Quat) {
        self.normal = rotation.rotate(self.normal)
    }

    fn print(&self) {
        println!(
            "Plan de point {:?} et de normale {:?}",
            self.point, self.normal
        );
    }
}

// Triangle stuff

pub struct Triangle {
    point_1: Vect3,
    normal: Vect3,
    vect_1: Vect3,
    vect_2: Vect3,
    material: Material,
}

impl Triangle {
    pub fn new(point_1: Vect3, point_2: Vect3, point_3: Vect3, material: Material) -> Self {
        let vect_1 = point_2 - point_1;
        let vect_2 = point_3 - point_1;
        let normal = vect_2.prod(vect_1).normalize();
        Self {
            point_1,
            normal,
            vect_1,
            vect_2,
            material,
        }
    }
}

impl Object3D for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if (ray.direction * self.normal).abs() <= 1e-5 {
            return None;
        }

        let vect_d = ray.start - self.point_1;
        let det_den = det_from_3_vects(-1.0 * ray.direction, self.vect_1, self.vect_2);

        let u = det_from_3_vects(-1.0 * ray.direction, vect_d, self.vect_2) / det_den;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let v = det_from_3_vects(-1.0 * ray.direction, self.vect_1, vect_d) / det_den;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let distance = det_from_3_vects(vect_d, self.vect_1, self.vect_2) / det_den;
        if distance > f64::EPSILON {
            let point = ray.start + distance * ray.direction;
            Some(Intersection::new(
                distance,
                self.material,
                point,
                self.normal,
            ))
        } else {
            None
        }
    }

    fn rotate(&mut self, rotation: Quat) {}

    fn print(&self) {
        println!(
            "Triangle de point {:?} et de normale {:?}",
            self.point_1, self.normal
        );
    }
}

// Complex creators

pub struct Cube {
    transform: Transform,
    triangles: Vec<Triangle>,
}

impl Cube {
    pub fn new(position: Vect3, rotation: Quat, size: f64, material: Material) -> Self {
        let mut triangles = vec![];

        let point_1 = rotation.rotate(position);
        let point_2 = rotation.rotate(position + size * Vect3::RIGHT);
        let point_3 = rotation.rotate(position + size * (Vect3::RIGHT + Vect3::FORWARD));
        let point_4 = rotation.rotate(position + size * Vect3::FORWARD);
        let point_5 = rotation.rotate(position + size * Vect3::UP);
        let point_6 = rotation.rotate(position + size * (Vect3::UP + Vect3::RIGHT));
        let point_7 =
            rotation.rotate(position + size * (Vect3::UP + Vect3::RIGHT + Vect3::FORWARD));
        let point_8 = rotation.rotate(position + size * (Vect3::UP + Vect3::FORWARD));

        triangles.push(Triangle::new(point_1, point_2, point_6, material)); // arrière
        triangles.push(Triangle::new(point_1, point_6, point_5, material));

        triangles.push(Triangle::new(point_2, point_3, point_7, material)); // droite
        triangles.push(Triangle::new(point_2, point_7, point_6, material));

        triangles.push(Triangle::new(point_3, point_4, point_8, material)); // avant
        triangles.push(Triangle::new(point_3, point_8, point_7, material));

        triangles.push(Triangle::new(point_4, point_1, point_5, material)); // gauche
        triangles.push(Triangle::new(point_4, point_5, point_8, material));

        triangles.push(Triangle::new(point_5, point_6, point_7, material)); // haut
        triangles.push(Triangle::new(point_5, point_7, point_8, material));

        triangles.push(Triangle::new(point_1, point_4, point_3, material)); // bas
        triangles.push(Triangle::new(point_1, point_3, point_2, material));

        Self {
            transform: Transform::new(position, rotation),
            triangles,
        }
    }
}

impl Object3D for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.triangles
            .iter()
            .filter_map(|triangle| triangle.intersect(ray))
            .filter(|inter| inter.distance.is_finite() && inter.distance > 1e-5)
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }

    fn print(&self) {
        println!("Cube");
    }

    fn rotate(&mut self, rotation: Quat) {}
}
