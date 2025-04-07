use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use super::{
    color::ColorRBG,
    objs::Object3D,
    position::{Angle, Quat, Transform, Vect3},
};

// Scene stuff

pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Object3D>>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new(camera: Camera, objects: Vec<Box<dyn Object3D>>, lights: Vec<Light>) -> Scene {
        Scene {
            camera,
            objects,
            lights,
        }
    }

    pub fn render(&mut self) {
        let camera_pos = self.camera.transform.get_pos();

        let camera_axis = (
            self.camera.transform.get_x_axis(),
            self.camera.transform.get_y_axis(),
            self.camera.transform.get_z_axis(),
        );

        for x in 0..self.camera.image.get_width() {
            for y in 0..self.camera.image.get_height() {
                // We first cast a ray from the camera trough the pixel
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

                // We then take the closest intersection
                if let Some(inter) = closest_intersection {
                    let mut final_color = ColorRBG::BLACK;
                    let bias = 1e-4; // introduced to avoid self intersection (BUG #1)

                    // We go trough all the lights, casting a ray from the intersection point to the light
                    for light in &self.lights {
                        let light_origin = inter.point + bias * inter.normal;
                        let light_direction = (light.transform.get_pos() - inter.point).normalize();
                        let light_ray = Ray::new(light_origin, light_direction);

                        let is_lighten = !self.objects.iter().any(|object| {
                            matches!(object.intersect(&light_ray), Some(intersection) if intersection.distance > bias)
                        });

                        // If not, we calculate the light with Phong model
                        if is_lighten {
                            final_color = final_color
                                + ((light_direction * inter.normal) * light.color * inter.color);
                        }
                    }
                    self.camera.image.set_pixel(x, y, final_color.rgb());
                }
            }
        }

        // self.camera.image.save_as_file(filename)?;
        // Ok(())
    }

    pub fn save_image(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        self.camera.image.save_as_file(filename)?;
        Ok(())
    }
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

// Light stuff

pub struct Light {
    transform: Transform,
    color: ColorRBG,
}

impl Light {
    pub fn new(position: Vect3, color: ColorRBG) -> Light {
        Light {
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
