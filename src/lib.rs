pub mod mods;
use mods::{
    camera::{Camera, Light}, // vect::Vect3,
    color::ColorRBG,
    funcs::{self, Angle},
    img,
    objs::{Object3D, Sphere},
    position::{Quat, Vect3},
};

pub fn create_basic_text_image() {
    let w: u32 = 400;
    let h: u32 = 150;

    let r: i64 = 10;
    let pos: (i64, i64) = (24, 32);

    let mut img_rgb = img::ImageRGB::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let x_i = x as i64;
            let y_i = y as i64;
            if funcs::is_in_circle(&pos, &r, &x_i, &y_i) {
                let value = funcs::gradient_from_center(&pos, &r, &x_i, &y_i);
                img_rgb.set_pixel(x as usize, y as usize, (0, 0, value));
            }
        }
    }
    img_rgb.save_as_file("image_RGB").unwrap();
}

pub fn basic_3d_sphere() {
    let camera = Camera::build(
        Vect3::new(0.0, 0.0, 20.0),
        Quat::from_axis_angle_deg(Vect3::UP, 180.0),
        10.0,
        Angle::from_deg(50.0),
        1920,
        1080,
    );

    let objects: Vec<Box<dyn Object3D>> = vec![
        Box::new(Sphere::new(Vect3::ZERO, 3.0, ColorRBG::WHITE)),
        Box::new(Sphere::new(Vect3::new(1.0, 1.0, 4.0), 0.5, ColorRBG::GREEN)),
        Box::new(Sphere::new(Vect3::new(-1.0, 1.0, 4.0), 0.5, ColorRBG::BLUE)),
        Box::new(Sphere::new(Vect3::new(1.0, -1.0, 4.0), 0.5, ColorRBG::RED)),
        Box::new(Sphere::new(
            Vect3::new(-1.0, -1.0, 4.0),
            0.5,
            ColorRBG::PINK,
        )),
    ];

    let lights: Vec<Light> = vec![
        Light::new(Vect3::new(0.5, 0.5, 10.0), ColorRBG::WHITE),
        //Light::new(Vect3::new(-5.0, -5.0, 7.0), ColorRBG::WHITE),
    ];

    camera.render(objects, lights, "test_RT_3");

    // TODO Mettre en place un langage pour donner les paramètres
    // Voir : example.rtp
}
