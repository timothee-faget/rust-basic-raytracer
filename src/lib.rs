pub mod mods;
use mods::{
    color::ColorRBG,
    funcs::Angle,
    objs::{Object3D, Sphere},
    position::{Quat, Vect3},
    render::{Camera, Light, Scene},
};

pub fn basic_ray_tracing() {
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
        Box::new(Sphere::new(Vect3::new(1.0, 3.0, 4.0), 0.5, ColorRBG::GREEN)),
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
        Light::new(Vect3::new(-5.0, -5.0, 7.0), ColorRBG::WHITE),
    ];

    let mut scene = Scene::new(camera, objects, lights);
    let _ = scene.render("test_scene");
}
