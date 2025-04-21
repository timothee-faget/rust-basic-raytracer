use std::cmp::Ordering;

use super::{
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

pub trait Object3D: Send + Sync {
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
        let dot = ray.direction * self.normal;
        if dot.abs() <= 1e-5 {
            return None;
        }

        let distance = -((ray.start - self.point) * self.normal) / dot;

        if distance > 0.0 {
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
        let edge1 = self.vect_1;
        let edge2 = self.vect_2;

        let h = ray.direction.prod(edge2);
        let a = edge1 * h;

        // Rayon parallèle au triangle ?
        if a.abs() < 1e-5 {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.start - self.point_1;
        let u = f * s * h;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.prod(edge1);
        let v = f * ray.direction * q;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2 * q;
        if t > f64::EPSILON {
            let point = ray.start + ray.direction * t;
            Some(Intersection::new(t, self.material, point, self.normal))
        } else {
            None
        }
    }

    fn rotate(&mut self, rotation: Quat) {
        todo!("{:?}", rotation);
    }

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
        let mut closest_hit: Option<Intersection> = None;
        let mut closest_distance = f64::INFINITY;

        for triangle in &self.triangles {
            if let Some(hit) = triangle.intersect(ray) {
                let dist = hit.distance;
                if dist > 1e-5 && dist.is_finite() && dist < closest_distance {
                    closest_distance = dist;
                    closest_hit = Some(hit);
                }
            }
        }

        closest_hit
    }

    fn print(&self) {
        println!("Cube");
    }

    fn rotate(&mut self, rotation: Quat) {
        todo!("{:?} {:?}", rotation, self.transform.get_pos());
    }
}

// Object Enum

#[derive(Debug)]
pub enum Object {
    Sphere(Transform, f64, Material),
    Plane(Vect3, Vect3, Material),
    Triangle(Vect3, Vect3, Vect3, Vect3, Material),
}

impl Object {
    pub fn new_sphere(transform: Transform, radius: f64, material: Material) -> Self {
        Self::Sphere(transform, radius, material)
    }

    pub fn new_plane(point: Vect3, normal: Vect3, material: Material) -> Self {
        Self::Plane(point, normal, material)
    }

    pub fn new_triangle(
        point_1: Vect3,
        point_2: Vect3,
        point_3: Vect3,
        material: Material,
    ) -> Self {
        let vect_1 = point_2 - point_1;
        let vect_2 = point_3 - point_1;
        let normal = vect_2.prod(vect_1).normalize();
        Self::Triangle(point_1, normal, vect_1, vect_2, material)
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Object::Sphere(t, r, m) => {
                let a = ray.direction * ray.direction;
                let b = 2.0 * (ray.direction * (ray.start - t.get_pos()));
                let c = (ray.start - t.get_pos()) * (ray.start - t.get_pos()) - r * r;

                let (t1, t2) = solve_quadratic(a, b, c)?;

                let distance = match (t1 >= 0.0, t2 >= 0.0) {
                    (true, true) => t1.min(t2),
                    (true, false) => t1,
                    (false, true) => t2,
                    (false, false) => return None,
                };

                let point = ray.start + distance * ray.direction;
                let normal = (point - t.get_pos()).normalize();
                Some(Intersection::new(distance, *m, point, normal))
            }
            Object::Plane(p, n, m) => {
                let dot = ray.direction * *n;
                if dot.abs() <= 1e-5 {
                    return None;
                }

                let distance = -((ray.start - *p) * *n) / dot;

                if distance > 0.0 {
                    let point = ray.start + distance * ray.direction;
                    Some(Intersection::new(distance, *m, point, *n))
                } else {
                    None
                }
            }
            Object::Triangle(p, n, v1, v2, m) => {
                let edge1 = *v1;
                let edge2 = *v2;

                let h = ray.direction.prod(edge2);
                let a = edge1 * h;

                if a.abs() < 1e-5 {
                    return None;
                }

                let f = 1.0 / a;
                let s = ray.start - *p;
                let u = f * s * h;
                if !(0.0..=1.0).contains(&u) {
                    return None;
                }

                let q = s.prod(edge1);
                let v = f * ray.direction * q;
                if v < 0.0 || u + v > 1.0 {
                    return None;
                }

                let t = f * edge2 * q;
                if t > f64::EPSILON {
                    let point = ray.start + ray.direction * t;
                    Some(Intersection::new(t, *m, point, *n))
                } else {
                    None
                }
            }
        }
    }
}
