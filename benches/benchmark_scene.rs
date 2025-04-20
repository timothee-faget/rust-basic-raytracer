use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracer::mods::{
    color::ColorRBG,
    funcs::LCG,
    parser::Parser,
    position::{Angle, Quat, Vect3},
    render::{Camera, ImageRGB, Material, Ray},
};

pub fn bench_mat(c: &mut Criterion) {
    let mat = Material::new(
        ColorRBG::BLACK,
        ColorRBG::BLACK,
        ColorRBG::BLACK,
        0.0,
        1.0,
        1.0,
    );

    c.bench_function("mat_get_emited", |b| b.iter(|| mat.get_emited_light()));
}

pub fn bench_camera(c: &mut Criterion) {
    let camera = Camera::new(
        Vect3::ZERO,
        Quat::identity(),
        10.0,
        Angle::from_deg(55.0),
        ImageRGB::new(100, 100),
    );
    let camera_axis = (
        camera.transform.get_x_axis(),
        camera.transform.get_y_axis(),
        camera.transform.get_z_axis(),
    );

    c.bench_function("camera_get_direction", |b| {
        b.iter(|| camera.get_ray_direction(camera_axis, 10, 10))
    });
}

pub fn bench_image(c: &mut Criterion) {
    let mut image = ImageRGB::new(100, 100);

    c.bench_function("image_get_width", |b| b.iter(|| image.get_width()));
    c.bench_function("image_get_height", |b| b.iter(|| image.get_height()));
    c.bench_function("image_get_pixels", |b| b.iter(|| image.get_pixel_count()));
    c.bench_function("image_save_as_file", |b| {
        b.iter(|| image.save_as_file("BENCH"))
    });
}

pub fn bench_scene(c: &mut Criterion) {
    let mut parser = Parser::build("scenes/bench_scene.rtp").unwrap();
    let mut scene = parser.parse_scene();

    let ray = Ray::new(Vect3::new(0.0, 0.0, 20.0), Vect3::FORWARD);
    c.bench_function("scene_trace", |b| {
        b.iter(|| scene.trace(&ray, &mut LCG::new(123456789), 0))
    });
    c.bench_function("scene_render", |b| b.iter(|| scene.render_bounces()));
}

criterion_group!(
    benches_scene,
    bench_mat,
    bench_camera,
    bench_image,
    bench_scene
);
criterion_main!(benches_scene);
