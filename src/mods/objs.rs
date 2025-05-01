use core::f64;

use super::{
    funcs::solve_quadratic,
    image::ImageRGB,
    material::Material,
    position::{Angle, Quat, Transform, Vect3},
    ray::{Intersection, Ray},
};

/// Camera implementation
#[derive(Clone)]
pub struct Camera {
    pub transform: Transform,
    focal: f64,
    fov: Angle,
    pub image: ImageRGB,
}

impl Camera {
    /// New Camera constructor
    pub fn new(position: Vect3, rotation: Quat, focal: f64, fov: Angle, image: ImageRGB) -> Camera {
        Camera {
            transform: Transform::new(position, rotation),
            focal,
            fov,
            image,
        }
    }

    /// Build Camera by creating its image
    pub fn build(
        position: Vect3,
        rotation: Quat,
        focal: f64,
        fov: Angle,
        w: u32,
        h: u32,
    ) -> Camera {
        Camera {
            transform: Transform::new(position, rotation),
            focal,
            fov,
            image: ImageRGB::new(w, h),
        }
    }

    /// Set Image resolution
    pub fn set_image_resolution(&mut self, w: u32, h: u32) {
        self.image = ImageRGB::new(w, h);
    }

    /// Get ray direction for a given pixel
    pub fn get_ray_direction(
        &self,
        camera_axis: (Vect3, Vect3, Vect3),
        x: usize,
        y: usize,
    ) -> Vect3 {
        let w = 2.0 * (self.fov / 2.0).tan() * self.focal;
        let h = (self.image.get_height() as f64 / self.image.get_width() as f64) * w;
        let alpha = w / (self.image.get_width() as f64);
        let coeff_a = -(x as f64) * alpha + w / 2.0;
        let coeff_b = -(y as f64) * alpha + h / 2.0;
        (coeff_a * camera_axis.0 + coeff_b * camera_axis.1 + self.focal * camera_axis.2).normalize()
    }
}

/// Sphere implementation
pub struct Sphere {
    pub transform: Transform,
    pub radius: f64,
    material: Material,
}

impl Sphere {
    /// New Sphere constructor
    pub fn new(position: Vect3, radius: f64, material: Material) -> Sphere {
        Sphere {
            transform: Transform::new(position, Quat::identity()),
            radius,
            material,
        }
    }
}

impl Sphere {
    /// Sphere instersector
    pub fn intersect(&self, ray: &Ray, min_distance: f64) -> Option<Intersection> {
        let rd = ray.get_dir();
        let rs = ray.get_start();
        let pos = self.transform.get_pos();
        let rs_pos = rs - pos;

        let a = rd * rd;
        let b = 2.0 * (rd * rs_pos);
        let c = rs_pos * rs_pos - self.radius * self.radius;

        let (t1, t2) = solve_quadratic(a, b, c)?;

        let distance = match (t1 >= 0.0, t2 >= 0.0) {
            (true, true) => t1.min(t2),
            (true, false) => t1,
            (false, true) => t2,
            (false, false) => return None,
        };

        if distance < min_distance {
            let point = rs + distance * rd;
            let normal = (point - pos).normalize();
            Some(Intersection::new(distance, self.material, point, normal))
        } else {
            None
        }
    }

    /// Get sphere's material
    pub fn get_mat(&self) -> &Material {
        &self.material
    }
}

/// Plane implementation
pub struct Plane {
    point: Vect3,
    normal: Vect3,
    material: Material,
}

impl Plane {
    /// New Plane constructor
    pub fn new(point: Vect3, normal: Vect3, material: Material) -> Self {
        Self {
            point,
            normal: normal.normalize(),
            material,
        }
    }
}

impl Plane {
    /// Plane instersector
    #[inline]
    pub fn intersect(&self, ray: &Ray, min_distance: f64) -> Option<Intersection> {
        let rd = ray.get_dir();
        let rs = ray.get_start();

        let dot = rd * self.normal;
        if dot.abs() <= 1e-5 {
            return None;
        }

        let distance = -((rs - self.point) * self.normal) / dot;

        if distance > 0.0 && distance < min_distance {
            let point = rs + distance * rd;
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

    /// Get Plane's material
    pub fn get_mat(&self) -> &Material {
        &self.material
    }
}

/// Triangle implementation
pub struct Triangle {
    point_1: Vect3,
    normal: Vect3,
    vect_1: Vect3,
    vect_2: Vect3,
    material: Material,
}

impl Triangle {
    /// New Triangle constructor
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

    /// Get Triangle's material
    pub fn get_mat(&self) -> &Material {
        &self.material
    }

    /// Triangle instersector
    pub fn intersect(&self, ray: &Ray, min_distance: f64) -> Option<Intersection> {
        let rd = ray.get_dir();
        let rs = ray.get_start();

        let edge1 = self.vect_1;
        let edge2 = self.vect_2;

        let h = rd.prod(edge2);
        let a = edge1 * h;

        const EPSILON: f64 = 1e-8;
        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = rs - self.point_1;
        let u = f * s * h;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.prod(edge1);
        let v = f * rd * q;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2 * q;
        if t > EPSILON && t < min_distance {
            let point = rs + rd * t;
            Some(Intersection::new(t, self.material, point, self.normal))
        } else {
            None
        }
    }

    /// Rotate Triangle with quaternion
    pub fn rotate(&mut self, rotation: Quat, position: Vect3) {
        self.point_1 = rotation.rotate(self.point_1 + position);
        self.normal = rotation.rotate(self.normal + position);
        self.vect_1 = rotation.rotate(self.vect_1 + position);
        self.vect_2 = rotation.rotate(self.vect_2 + position);
    }

    /// Set Triangle's material
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

/// Creates triangles for a cube
pub fn create_cube_triangles(
    position: Vect3,
    rotation: Quat,
    size: f64,
    material: Material,
) -> Vec<Triangle> {
    let mut triangles = vec![];

    let point_1 = rotation.rotate(position);
    let point_2 = rotation.rotate(position + size * Vect3::RIGHT);
    let point_3 = rotation.rotate(position + size * (Vect3::RIGHT + Vect3::FORWARD));
    let point_4 = rotation.rotate(position + size * Vect3::FORWARD);
    let point_5 = rotation.rotate(position + size * Vect3::UP);
    let point_6 = rotation.rotate(position + size * (Vect3::UP + Vect3::RIGHT));
    let point_7 = rotation.rotate(position + size * (Vect3::UP + Vect3::RIGHT + Vect3::FORWARD));
    let point_8 = rotation.rotate(position + size * (Vect3::UP + Vect3::FORWARD));

    triangles.push(Triangle::new(point_1, point_2, point_6, material)); // back
    triangles.push(Triangle::new(point_1, point_6, point_5, material));

    triangles.push(Triangle::new(point_2, point_3, point_7, material)); // right
    triangles.push(Triangle::new(point_2, point_7, point_6, material));

    triangles.push(Triangle::new(point_3, point_4, point_8, material)); // front
    triangles.push(Triangle::new(point_3, point_8, point_7, material));

    triangles.push(Triangle::new(point_4, point_1, point_5, material)); // left
    triangles.push(Triangle::new(point_4, point_5, point_8, material));

    triangles.push(Triangle::new(point_5, point_6, point_7, material)); // up
    triangles.push(Triangle::new(point_5, point_7, point_8, material));

    triangles.push(Triangle::new(point_1, point_4, point_3, material)); // down
    triangles.push(Triangle::new(point_1, point_3, point_2, material));

    triangles
}
