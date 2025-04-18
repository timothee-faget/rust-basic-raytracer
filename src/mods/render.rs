use core::f64;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use rayon::prelude::*;

use crate::mods::color::ColorRBGOF;

use super::color::lerp_color;
use super::constants::BIAS;
use super::constants::MAX_BOUNCES;
use super::constants::RENDER_ITERATIONS;
use super::objs::Intersection;
use super::{
    color::ColorRBG,
    funcs::LCG,
    objs::Object3D,
    position::{lerp, Angle, Quat, Transform, Vect3},
};

// Scene stuff

pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Object3D>>,
}

impl Scene {
    pub fn new(camera: Camera, objects: Vec<Box<dyn Object3D>>) -> Scene {
        Scene { camera, objects }
    }

    pub fn render_bounces(&mut self) {
        println!("== Rendering scene");
        let camera_pos = self.camera.transform.get_pos();
        let camera_axis = (
            self.camera.transform.get_x_axis(),
            self.camera.transform.get_y_axis(),
            self.camera.transform.get_z_axis(),
        );

        let width = self.camera.image.get_width();
        let height = self.camera.image.get_height();
        let mut acc_buffer = vec![ColorRBGOF::BLACK; width * height];

        // Créer une structure Scene partageable entre threads
        let scene = &self;

        for f in 0..RENDER_ITERATIONS {
            // Paralléliser par pixel - directement dans les closures
            let all_pixels: Vec<(usize, usize)> = (0..width)
                .flat_map(|x| (0..height).map(move |y| (x, y)))
                .collect();

            let frame_results: Vec<(usize, usize, ColorRBG)> = all_pixels
                .into_par_iter()
                .map(|(x, y)| {
                    let pixel_seed = 123456789_u64
                        .wrapping_add(f as u64 * 0xA24BAED4)
                        .wrapping_add((y * width + x) as u64 * 0x9E3779B9)
                        .wrapping_mul(74747_u64); // Ajouter un multiplicateur pour améliorer la dispersion
                    let mut local_randomizer = LCG::new(pixel_seed);

                    let ray = Ray::new(
                        camera_pos,
                        scene.camera.get_ray_direction(camera_axis, x, y),
                    );
                    let color = scene.trace(&ray, &mut local_randomizer, 0);

                    (x, y, color)
                })
                .collect(); // Accumuler les résultats séquentiellement
            for (x, y, color) in frame_results {
                let idx = y * width + x;
                acc_buffer[idx] = acc_buffer[idx] + color;
            }
        }

        // Finaliser l'image - de manière séquentielle pour éviter les problèmes de mutabilité
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let avg_color = ((1.0 / RENDER_ITERATIONS as f64) * acc_buffer[idx]).to_rgb();
                self.camera.image.set_pixel(x, y, avg_color.rgb());
            }
        }
    }

    pub fn trace(&self, ray: &Ray, randomizer: &mut LCG, bounce: u32) -> ColorRBG {
        if bounce > MAX_BOUNCES {
            return ColorRBG::BLACK;
        }

        let mut closest_intersection: Option<Intersection> = None;
        let mut min_distance = f64::INFINITY;

        for object in &self.objects {
            if let Some(hit) = object.intersect(ray) {
                if hit.distance > 1e-5 && hit.distance < min_distance {
                    min_distance = hit.distance;
                    closest_intersection = Some(hit);
                }
            }
        }

        if let Some(inter) = closest_intersection {
            let is_specular = inter.material.specular_prob >= randomizer.next_f64();

            let dot = ray.direction * inter.normal;
            let specular_dir = (ray.direction - 2.0 * inter.normal * dot).normalize();

            let diffuse_dir =
                (inter.normal + randomizer.next_normal_vect3(inter.normal)).normalize();
            let ray_origin = inter.point + inter.normal * BIAS;

            let ray_dir = lerp(
                diffuse_dir,
                specular_dir,
                inter.material.smoothness * (is_specular as u8 as f64),
            )
            .normalize();
            let new_ray = Ray::new(ray_origin, ray_dir);

            let emitted = inter.material.get_emited_light();

            let reflectance = lerp_color(
                inter.material.color,
                inter.material.specular_color,
                is_specular as u8 as f64,
            );

            // Estimation d'importance / Russian roulette
            let p = reflectance.max_component().clamp(0.1, 1.0);
            if randomizer.next_f64() >= p {
                return emitted;
            }

            let next_bounce_light = self.trace(&new_ray, randomizer, bounce + 1);
            emitted + (1.0 / p) * (reflectance * next_bounce_light)
        } else {
            ColorRBG::BLACK
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Object3D>) {
        self.objects.push(object);
    }

    pub fn save_image(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        println!("== Saving scene");
        self.camera.image.save_as_file(filename)?;
        Ok(())
    }
}

// Camera stuff

#[derive(Clone)]
pub struct Camera {
    pub transform: Transform,
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

// Material stuff

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: ColorRBG,
    pub emission_color: ColorRBG,
    pub specular_color: ColorRBG,
    pub emission_strengh: f64,
    pub smoothness: f64,
    pub specular_prob: f64,
}

impl Material {
    pub fn new(
        color: ColorRBG,
        emission_color: ColorRBG,
        specular_color: ColorRBG,
        emission_strengh: f64,
        smoothness: f64,
        specular_prob: f64,
    ) -> Self {
        Self {
            color,
            emission_color,
            specular_color,
            emission_strengh,
            smoothness,
            specular_prob,
        }
    }

    #[inline]
    pub fn get_emited_light(&self) -> ColorRBG {
        self.emission_strengh.min(1.0) * self.emission_color
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

    #[inline]
    pub fn get_width(&self) -> usize {
        self.data[0].len()
    }

    #[inline]
    pub fn get_height(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn get_pixel_count(&self) -> usize {
        self.data.len() * self.data[0].len()
    }

    #[inline]
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
