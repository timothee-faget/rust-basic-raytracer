pub mod mods;
use mods::{
    color::ColorRBG,
    objs::{Cube, Object3D, Plane, Sphere},
    parser::Parser,
    position::{Angle, Quat, Vect3},
    render::{Camera, Light, Material, RectLight, Scene},
};

pub fn basic_ray_tracing() {
    let mut parser = Parser::new("scenes/test_surfacic.rtp");
    let mut scene_parser = parser.parse_scene();

    scene_parser.add_light(Box::new(RectLight::new(
        Vect3::new(-3.0, 4.0, -3.0),
        Quat::identity(),
        0.5 * Vect3::RIGHT,
        0.5 * Vect3::FORWARD,
        ColorRBG::PINK,
    )));
    scene_parser.add_light(Box::new(RectLight::new(
        Vect3::new(3.0, 4.0, -3.0),
        Quat::identity(),
        0.5 * Vect3::RIGHT,
        0.5 * Vect3::FORWARD,
        ColorRBG::LIGHT_BLUE,
    )));
    scene_parser.add_light(Box::new(RectLight::new(
        Vect3::new(-3.0, 4.0, 3.0),
        Quat::identity(),
        0.5 * Vect3::RIGHT,
        0.5 * Vect3::FORWARD,
        ColorRBG::GREEN,
    )));
    scene_parser.add_light(Box::new(RectLight::new(
        Vect3::new(3.0, 4.0, 3.0),
        Quat::identity(),
        0.5 * Vect3::RIGHT,
        0.5 * Vect3::FORWARD,
        ColorRBG::RED,
    )));

    let mat = Material::new(
        ColorRBG::new(0.7, 0.7, 0.7),
        ColorRBG::new(0.7, 0.7, 0.7),
        ColorRBG::new(1.0, 1.0, 1.0),
        50.0,
    );

    let _mat_2 = Material::new(
        ColorRBG::new(0.0, 1.0, 0.0),
        ColorRBG::new(0.0, 0.8, 0.0),
        ColorRBG::new(1.0, 1.0, 1.0),
        50.0,
    );

    scene_parser.add_object(Box::new(Sphere::new(Vect3::new(0.0, 1.0, 0.0), 1.0, mat)));
    scene_parser.add_object(Box::new(Cube::new(
        Vect3::new(-1.5, 0.0, 0.0),
        Quat::from_axis_angle_deg(Vect3::UP, 45.0),
        1.0,
        mat,
    )));

    scene_parser.render();
    let _ = scene_parser.save_image("outputs/test_surfacique");
    //let camera = Camera::build(
    //    Vect3::new(0.0, 4.0, 20.0),
    //    Quat::new(0.0, Vect3::new(0.0, 1.0, -0.08)),
    //    10.0,
    //    Angle::from_deg(45.0),
    //    4000,
    //    3000,
    //);
    //
    //let lights = vec![
    //    Light::new(Vect3::new(10.0, 10.0, 10.0), ColorRBG::WHITE),
    //    Light::new(Vect3::new(-10.0, 10.0, 10.0), ColorRBG::WHITE),
    //    Light::new(Vect3::new(10.0, 10.0, -10.0), ColorRBG::WHITE),
    //    Light::new(Vect3::new(-10.0, 10.0, -10.0), ColorRBG::WHITE),
    //];
    //
    //let mat = Material::new(
    //    ColorRBG::new(0.0, 1.0, 0.0),
    //    ColorRBG::new(0.0, 1.0, 1.0),
    //    ColorRBG::new(0.5, 0.5, 0.5),
    //    50.0,
    //);
    //
    //let mat_plan = Material::new(
    //    ColorRBG::new(0.0, 1.0, 0.0),
    //    ColorRBG::new(0.5, 0.5, 0.5),
    //    ColorRBG::new(0.0, 0.5, 0.0),
    //    50.0,
    //);
    //
    //let mat_sphere = Material::new(
    //    ColorRBG::new(0.0, 1.0, 0.0),
    //    ColorRBG::new(0.5, 0.5, 0.0),
    //    ColorRBG::new(0.5, 0.5, 0.0),
    //    50.0,
    //);
    //
    //let objects: Vec<Box<dyn Object3D>> = vec![
    //    Box::new(Cube::new(
    //        Vect3::new(-3.0, -3.0, -3.0),
    //        Quat::from_axis_angle_deg(Vect3::UP, 35.0),
    //        6.0,
    //        mat,
    //    )),
    //    Box::new(Plane::new(
    //        Vect3::new(-3.0, -3.0, -3.0),
    //        Vect3::UP,
    //        mat_plan,
    //    )),
    //    Box::new(Sphere::new(Vect3::new(3.0, 3.0, 3.0), 0.2, mat_sphere)),
    //    Box::new(Sphere::new(Vect3::new(3.0, 3.0, -3.0), 0.2, mat_sphere)),
    //    Box::new(Sphere::new(Vect3::new(3.0, -3.0, 3.0), 0.2, mat_sphere)),
    //    Box::new(Sphere::new(Vect3::new(3.0, -3.0, -3.0), 0.2, mat_sphere)),
    //    Box::new(Sphere::new(Vect3::new(-3.0, 3.0, 3.0), 0.2, mat_sphere)),
    //    Box::new(Sphere::new(Vect3::new(-3.0, 3.0, -3.0), 0.2, mat_sphere)),
    //    Box::new(Sphere::new(Vect3::new(-3.0, -3.0, 3.0), 0.2, mat_sphere)),
    //    Box::new(Sphere::new(Vect3::new(-3.0, -3.0, -3.0), 0.2, mat_sphere)),
    //];
    //
    //let mut scene = Scene::new(camera, objects, lights);
    //
    //scene.render();
    //let _ = scene.save_image("outputs/test_cube_1");
}
