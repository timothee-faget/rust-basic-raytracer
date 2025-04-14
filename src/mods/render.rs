use core::f64;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use super::{
    color::ColorRBG,
    objs::{Intersection, Object3D},
    position::{self, Angle, Quat, Transform, Vect3},
};

// Scene stuff

pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Object3D>>,
    lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn new(
        camera: Camera,
        objects: Vec<Box<dyn Object3D>>,
        lights: Vec<Box<dyn Light>>,
    ) -> Scene {
        Scene {
            camera,
            objects,
            lights,
        }
    }

    pub fn render(&mut self) {
        println!("== Rendering scene");
        let camera_pos = self.camera.transform.get_pos();

        let camera_axis = (
            self.camera.transform.get_x_axis(),
            self.camera.transform.get_y_axis(),
            self.camera.transform.get_z_axis(),
        );

        for x in 0..self.camera.image.get_width() {
            for y in 0..self.camera.image.get_height() {
                let ray = Ray::new(camera_pos, self.camera.get_ray_direction(camera_axis, x, y));

                let closest_intersection = self
                    .objects
                    .iter()
                    .filter_map(|object| object.intersect(&ray))
                    .min_by(|a, b| {
                        a.distance
                            .partial_cmp(&b.distance)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });

                if let Some(inter) = closest_intersection {
                    let mut final_color = ColorRBG::BLACK;

                    for light in &self.lights {
                        final_color =
                            final_color + light.get_light(&inter, camera_pos, &self.objects);
                    }

                    self.camera.image.set_pixel(x, y, final_color.rgb());
                }
            }
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Object3D>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    pub fn save_image(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        println!("== Saving scene");
        self.camera.image.save_as_file(filename)?;
        Ok(())
    }
}

fn get_distance_coef_dif(d: f64) -> f64 {
    1.0 / (0.002 * d * d + 0.05 * d + 1.0)
}

fn get_distance_coef_amb(d: f64) -> f64 {
    1.0 / (2.0 * d * d + 2.0 * d + 1.0)
}

// Camera stuff

#[derive(Clone)]
pub struct Camera {
    transform: Transform,
    focal: f64,
    fov: Angle,
    image: ImageRGB,
}

impl Camera {
    pub fn new(position: Vect3, rotation: Quat, focal: f64, fov: Angle, image: ImageRGB) -> Camera {
        Camera {
            transform: Transform::new(position, rotation),
            focal,
            fov,
            image,
        }
    }

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

    fn get_ray_direction(&self, camera_axis: (Vect3, Vect3, Vect3), x: usize, y: usize) -> Vect3 {
        let w = 2.0 * (self.fov / 2.0).tan() * self.focal;
        let h = (self.image.get_height() as f64 / self.image.get_width() as f64) * w;
        let alpha = w / (self.image.get_width() as f64);
        let coeff_a = -(x as f64) * alpha + w / 2.0;
        let coeff_b = -(y as f64) * alpha + h / 2.0;
        (coeff_a * camera_axis.0 + coeff_b * camera_axis.1 + self.focal * camera_axis.2).normalize()
    }
}

// Material stuff

#[derive(Debug, Clone, Copy)]
pub struct Material {
    ambient: ColorRBG,
    diffuse: ColorRBG,
    specular: ColorRBG,
    shininess: f64,
}

impl Material {
    pub fn new(ambient: ColorRBG, diffuse: ColorRBG, specular: ColorRBG, shininess: f64) -> Self {
        Material {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn get_amb(&self) -> ColorRBG {
        self.ambient
    }

    pub fn get_dif(&self) -> ColorRBG {
        self.diffuse
    }

    pub fn get_spe(&self) -> ColorRBG {
        self.specular
    }

    pub fn get_shi(&self) -> f64 {
        self.shininess
    }
}

// Light stuff
pub trait Light {
    fn get_light(
        &self,
        inter: &Intersection,
        camera_pos: Vect3,
        objects: &Vec<Box<dyn Object3D>>,
    ) -> ColorRBG;
}

pub struct PointLight {
    transform: Transform,
    color: ColorRBG,
}

impl PointLight {
    pub fn new(position: Vect3, color: ColorRBG) -> Self {
        Self {
            transform: Transform::new(position, Quat::identity()),
            color,
        }
    }

    pub fn get_pos(&self) -> Vect3 {
        self.transform.get_pos()
    }

    pub fn get_color(&self) -> ColorRBG {
        self.color
    }
}

impl Light for PointLight {
    fn get_light(
        &self,
        inter: &Intersection,
        camera_pos: Vect3,
        objects: &Vec<Box<dyn Object3D>>,
    ) -> ColorRBG {
        let bias = 1e-4;
        let light_direction = (self.get_pos() - inter.point).normalize();
        let light_distance = (self.get_pos() - inter.point).norm();
        let light_ray = Ray::new(inter.point + bias * inter.normal, light_direction);

        let mut final_color = ColorRBG::BLACK;

        if !objects.iter().any(|object| {
            if let Some(intersection) = object.intersect(&light_ray) {
                intersection.distance > bias && intersection.distance < light_distance
            } else {
                false
            }
        }) {
            let reflect_direction = (2.0 * (light_direction * inter.normal) * inter.normal
                - light_direction)
                .normalize();
            let viewer_direction = (camera_pos - inter.point).normalize();

            final_color = final_color
                + get_distance_coef_dif(light_distance)
                    * ((light_direction * inter.normal) * self.color * inter.material.diffuse)
                + (reflect_direction * viewer_direction).powf(inter.material.shininess)
                    * self.color
                    * inter.material.specular; // SPECULAR PART
        }
        final_color + get_distance_coef_amb(light_distance) * self.color * inter.material.ambient
    }
}

pub struct RectLight {
    transform: Transform,
    vect_1: Vect3,
    vect_2: Vect3,
    color: ColorRBG,
}

impl RectLight {
    pub fn new(
        position: Vect3,
        rotation: Quat,
        vect_1: Vect3,
        vect_2: Vect3,
        color: ColorRBG,
    ) -> Self {
        Self {
            transform: Transform::new(position, rotation),
            vect_1,
            vect_2,
            color,
        }
    }
}

impl Light for RectLight {
    fn get_light(
        &self,
        inter: &Intersection,
        camera_pos: Vect3,
        objects: &Vec<Box<dyn Object3D>>,
    ) -> ColorRBG {
        let res = 10;
        let grid: Vec<f64> = (0..res + 1).map(|i| i as f64 / (res as f64)).collect();
        let coef = 1.0 / (res as f64 * res as f64);
        //println!("{:?}", grid);

        let mut final_color = ColorRBG::BLACK;
        for i in &grid {
            for j in &grid {
                let ech_point = self.transform.get_pos() + self.vect_1 * *i + self.vect_2 * *j;
                //println!("{:?}", ech_point);
                let bias = 1e-4;
                let light_direction = (ech_point - inter.point).normalize();
                let light_distance = (ech_point - inter.point).norm();
                let light_ray = Ray::new(inter.point + bias * inter.normal, light_direction);

                if !objects.iter().any(|object| {
                    if let Some(intersection) = object.intersect(&light_ray) {
                        intersection.distance > bias && intersection.distance < light_distance
                    } else {
                        false
                    }
                }) {
                    let reflect_direction = (2.0 * (light_direction * inter.normal) * inter.normal
                        - light_direction)
                        .normalize();
                    let viewer_direction = (camera_pos - inter.point).normalize();

                    final_color = final_color
                        + 1.0
                            * coef
                            * (get_distance_coef_dif(light_distance)
                                * ((light_direction * inter.normal)
                                    * self.color
                                    * inter.material.diffuse)
                                + (reflect_direction * viewer_direction)
                                    .powf(inter.material.shininess)
                                    * self.color
                                    * inter.material.specular); // SPECULAR PART
                }
                final_color = final_color
                    + 5.0
                        * coef
                        * get_distance_coef_amb(light_distance)
                        * self.color
                        * inter.material.ambient;
            }
        }
        //println!("{:?}", final_color);
        final_color
    }
}

// Ray stuff

pub struct Ray {
    pub start: Vect3,
    pub direction: Vect3,
}

impl Ray {
    pub fn new(start: Vect3, direction: Vect3) -> Ray {
        Ray { start, direction }
    }

    pub fn get_dir(&self) -> Vect3 {
        self.direction
    }
}

// Image stuff

#[derive(Clone)]
pub struct ImageRGB {
    data: Vec<Vec<(u8, u8, u8)>>,
}

impl ImageRGB {
    pub fn new(w: u32, h: u32) -> Self {
        let line = vec![(255, 255, 255); w as usize];
        ImageRGB {
            data: vec![line; h as usize],
        }
    }

    pub fn get_width(&self) -> usize {
        self.data[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.data.len()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: (u8, u8, u8)) {
        self.data[y][x] = value;
    }

    pub fn save_as_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(format!("{}.ppm", filename))?;
        let header = format!("P3\n{} {}\n255\n", self.data[0].len(), self.data.len());

        file.write_all(header.as_bytes())?;

        let mut content = String::new();
        for line in self.data.iter() {
            for el in line {
                content += format!("{} {} {}\n", el.0, el.1, el.2).as_str();
            }
        }
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
