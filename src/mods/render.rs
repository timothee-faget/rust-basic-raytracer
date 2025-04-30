use core::f64;
use std::error::Error;

use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::mods::funcs::s_to_hms;

use super::{
    color::{lerp_color, ColorRBG, ColorRBGOF},
    objs::{Camera, Plane, Sphere, Triangle},
    position::lerp,
    random::LCG,
    ray::{Intersection, Ray},
};

static BIAS: f64 = 1e-5;

/// Scene implementation
pub struct Scene {
    pub camera: Camera,
    pub spheres: Vec<Sphere>,
    pub planes: Vec<Plane>,
    pub triangles: Vec<Triangle>,
    render_iterations: usize,
    max_bounces: u32,
}

impl Scene {
    /// New Scene constructor
    pub fn new(
        camera: Camera,
        spheres: Vec<Sphere>,
        planes: Vec<Plane>,
        triangles: Vec<Triangle>,
    ) -> Scene {
        Scene {
            camera,
            spheres,
            planes,
            triangles,
            render_iterations: 10,
            max_bounces: 10,
        }
    }

    /// Render Scene
    pub fn render(&mut self, render_iterations: usize, max_bounces: u32, resolution: (u32, u32)) {
        print_render_info(render_iterations, max_bounces, resolution.0, resolution.1);
        self.render_iterations = render_iterations;
        self.max_bounces = max_bounces;
        self.camera.set_image_resolution(resolution.0, resolution.1);

        let camera_pos = self.camera.transform.get_pos();
        let camera_axis = (
            self.camera.transform.get_x_axis(),
            self.camera.transform.get_y_axis(),
            self.camera.transform.get_z_axis(),
        );

        let width = self.camera.image.get_width();
        let height = self.camera.image.get_height();
        let mut acc_buffer = vec![ColorRBGOF::BLACK; width * height];

        let scene = &self;
        let bar = ProgressBar::new(self.render_iterations as u64);
        bar.set_style(ProgressStyle::default_bar().template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} ({percent}%) | ETA: {eta}",
        ).unwrap().progress_chars("##-"));

        bar.inc(0);
        for f in 0..self.render_iterations {
            let all_pixels: Vec<(usize, usize)> = (0..width)
                .flat_map(|x| (0..height).map(move |y| (x, y)))
                .collect();

            let frame_results: Vec<(usize, usize, ColorRBG)> = all_pixels
                .into_par_iter()
                .map(|(x, y)| {
                    let pixel_seed = 123456789_u64
                        .wrapping_add(f as u64 * 0xA24BAED4)
                        .wrapping_add((y * width + x) as u64 * 0x9E3779B9)
                        .wrapping_mul(74747_u64);
                    let mut local_randomizer = LCG::new(pixel_seed);

                    let ray = Ray::new(
                        camera_pos,
                        scene.camera.get_ray_direction(camera_axis, x, y),
                    );
                    let color = scene.trace(&ray, &mut local_randomizer, 0);

                    (x, y, color)
                })
                .collect();
            for (x, y, color) in frame_results {
                let idx = y * width + x;
                acc_buffer[idx] = acc_buffer[idx] + color;
            }

            bar.inc(1);
        }

        //bar.finish_with_message("Rendu terminé!");
        let coeff = 1.0 / self.render_iterations as f64;
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let avg_color = (coeff * acc_buffer[idx]).to_rgb();
                self.camera.image.set_pixel(x, y, avg_color.rgb());
            }
        }

        let render_time = bar.elapsed();
        bar.finish_and_clear();
        println!(
            "      Rendered scene in {}",
            style(s_to_hms(render_time.as_secs_f64())).bold().white()
        );
    }

    /// Trace ray
    pub fn trace(&self, ray: &Ray, randomizer: &mut LCG, bounce: u32) -> ColorRBG {
        if bounce > self.max_bounces {
            return ColorRBG::BLACK;
        }

        let closest_intersection = self.get_intersection(ray);

        if let Some(inter) = closest_intersection {
            let is_specular = inter.material.specular_prob >= randomizer.next_f64();

            let rd = ray.get_dir();

            let dot = rd * inter.normal;
            let specular_dir = (rd - 2.0 * inter.normal * dot).normalize();

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

    /// Get Intersection of Ray with Scene's objects
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest_intersection: Option<Intersection> = None;
        let mut min_distance = f64::INFINITY;

        for sphere in &self.spheres {
            if let Some(hit) = sphere.intersect(ray, min_distance) {
                if hit.distance > BIAS && hit.distance < min_distance {
                    min_distance = hit.distance;
                    closest_intersection = Some(hit);
                }
            }
        }
        for plane in &self.planes {
            if let Some(hit) = plane.intersect(ray, min_distance) {
                if hit.distance > BIAS && hit.distance < min_distance {
                    min_distance = hit.distance;
                    closest_intersection = Some(hit);
                }
            }
        }

        for triangle in &self.triangles {
            if let Some(hit) = triangle.intersect(ray, min_distance) {
                if hit.distance > BIAS && hit.distance < min_distance {
                    min_distance = hit.distance;
                    closest_intersection = Some(hit);
                }
            }
        }
        closest_intersection
    }

    /// Prints Scene information
    pub fn get_info(&self) {
        println!("      Parsed scene containing :");
        println!(
            "        - {} speres",
            style(self.spheres.len()).bold().blue()
        );
        println!(
            "        - {} planes",
            style(self.planes.len()).bold().blue()
        );
        println!(
            "        - {} triangles",
            style(self.triangles.len()).bold().blue()
        );
    }

    /// Save Scene image to ppm file
    pub fn save_image(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        self.camera.image.save_as_ppm(filename)?;
        Ok(())
    }
}

/// Prints render information
fn print_render_info(ri: usize, mb: u32, w: u32, h: u32) {
    println!(
        "{} Rendering scene with these parameters :",
        style("[2/3]").bold().green()
    );
    println!("        - Iterations : {}", style(ri).bold().blue());
    println!("        - Max bounces : {}", style(mb).bold().blue());
    println!(
        "        - Image resolution : {} x {} pixels",
        style(w).bold().blue(),
        style(h).bold().blue()
    );
}
