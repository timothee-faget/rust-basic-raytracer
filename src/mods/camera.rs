use super::{
    color::ColorRBG,
    funcs::Angle,
    img::ImageRGB,
    objs::Object3D,
    position::{Quat, Transform, Vect3},
};

// Camera stuff

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

    fn get_ray_direction_2(&self, camera_axis: (Vect3, Vect3, Vect3), x: usize, y: usize) -> Vect3 {
        let w = 2.0 * (self.fov.get() / 2.0).tan() * self.focal;
        let h = (self.image.get_height() as f64 / self.image.get_width() as f64) * w;
        let alpha = w / (self.image.get_width() as f64);
        let coeff_a = -(x as f64) * alpha + w / 2.0;
        let coeff_b = -(y as f64) * alpha + h / 2.0;
        Vect3::new(
            coeff_a * camera_axis.0.x + coeff_b * camera_axis.1.x + self.focal * camera_axis.2.x,
            coeff_a * camera_axis.0.y + coeff_b * camera_axis.1.y + self.focal * camera_axis.2.y,
            coeff_a * camera_axis.0.z + coeff_b * camera_axis.1.z + self.focal * camera_axis.2.z,
        )
        .normalize()
    }

    pub fn render(mut self, objects: Vec<Box<dyn Object3D>>, lights: Vec<Light>, filename: &str) {
        let camera_pos = self.transform.get_pos();

        let camera_axis = (
            self.transform.get_x_axis(),
            self.transform.get_y_axis(),
            self.transform.get_z_axis(),
        );

        //println!("x_camera : {:?}", self.transform.get_x_axis());
        //println!("y_camera : {:?}", self.transform.get_y_axis());
        //println!("z_camera : {:?}", self.transform.get_z_axis());
        //println!(
        //    "Direction (1) pour P(400,600) : {:?}",
        //    self.get_ray_direction_2(camera_axis, 400, 600)
        //);

        for x in 0..self.image.get_width() {
            for y in 0..self.image.get_height() {
                // We first cast a ray from the camera trough the pixel
                let ray = Ray::new(camera_pos, self.get_ray_direction_2(camera_axis, x, y));

                let closest_intersection = objects
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

                    // We go trough all the lights, casting a ray from the intersection point to the light
                    for light in &lights {
                        let light_direction = (light.transform.get_pos() - inter.point).normalize();
                        let light_ray = Ray::new(inter.point, light_direction);

                        let is_lighten = !objects.iter().any(|object| {
                            matches!(object.intersect(&light_ray), Some(intersection) if intersection.distance > 0.0)
                        });

                        // If not, we calculate the light with Phong model
                        if is_lighten {
                            final_color = final_color
                                + ((light_direction * inter.normal) * light.color * inter.color);
                        }
                    }
                    self.image.set_pixel(x, y, final_color.rgb());
                }
            }
        }

        self.image.save_as_file(filename).unwrap();
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
