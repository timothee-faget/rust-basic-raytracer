use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracer::mods::{
    color::ColorRBG,
    objs::{Intersection, Object3D, Plane, Sphere, Triangle},
    position::Vect3,
    render::{NewMaterial, Ray},
};

pub fn bench_intersection(c: &mut Criterion) {
    let mat = NewMaterial::new(
        ColorRBG::BLACK,
        ColorRBG::BLACK,
        ColorRBG::BLACK,
        0.0,
        1.0,
        1.0,
    );
    let inter_test_1 = Intersection::new(1.0, mat, Vect3::ZERO, Vect3::UP);
    let inter_test_2 = Intersection::new(1.0, mat, Vect3::ZERO, Vect3::UP);

    c.bench_function("comparison", |b| b.iter(|| inter_test_1 > inter_test_2));
}

pub fn bench_sphere(c: &mut Criterion) {
    let mat = NewMaterial::new(
        ColorRBG::BLACK,
        ColorRBG::BLACK,
        ColorRBG::BLACK,
        0.0,
        1.0,
        1.0,
    );
    let sphere_test = Sphere::new(Vect3::ZERO, 1.0, mat);
    let ray_test_1 = Ray::new(Vect3::new(0.0, 0.0, -10.0), Vect3::FORWARD);
    let ray_test_2 = Ray::new(Vect3::new(0.0, 0.0, 10.0), Vect3::FORWARD);

    c.bench_function("sphere_intersection_1", |b| {
        b.iter(|| sphere_test.intersect(&ray_test_1))
    });
    c.bench_function("sphere_intersection_2", |b| {
        b.iter(|| sphere_test.intersect(&ray_test_2))
    });
}

pub fn bench_plane(c: &mut Criterion) {
    let mat = NewMaterial::new(
        ColorRBG::BLACK,
        ColorRBG::BLACK,
        ColorRBG::BLACK,
        0.0,
        1.0,
        1.0,
    );
    let plane_test = Plane::new(Vect3::ZERO, Vect3::UP, mat);
    let ray_test_1 = Ray::new(Vect3::new(0.0, 1.0, 0.0), Vect3::UP);
    let ray_test_2 = Ray::new(Vect3::new(0.0, 1.0, 0.0), Vect3::DOWN);

    c.bench_function("plane_intersection_1", |b| {
        b.iter(|| plane_test.intersect(&ray_test_1))
    });
    c.bench_function("plane_intersection_2", |b| {
        b.iter(|| plane_test.intersect(&ray_test_2))
    });
}

pub fn bench_triangle(c: &mut Criterion) {
    let mat = NewMaterial::new(
        ColorRBG::BLACK,
        ColorRBG::BLACK,
        ColorRBG::BLACK,
        0.0,
        1.0,
        1.0,
    );
    let triangle_test = Triangle::new(
        Vect3::new(1.0, 0.0, 0.0),
        Vect3::new(-1.0, 0.0, 0.0),
        Vect3::new(0.0, 1.0, 0.0),
        mat,
    );
    let ray_test_1 = Ray::new(Vect3::new(0.0, 0.0, 1.0), Vect3::FORWARD);
    let ray_test_2 = Ray::new(Vect3::new(0.0, 0.0, 1.0), Vect3::BACKWARD);

    c.bench_function("triangle_intersection_1", |b| {
        b.iter(|| triangle_test.intersect(&ray_test_1))
    });
    c.bench_function("triangle_intersection_2", |b| {
        b.iter(|| triangle_test.intersect(&ray_test_2))
    });
}

criterion_group!(
    benches_objs,
    bench_intersection,
    bench_sphere,
    bench_plane,
    bench_triangle
);
criterion_main!(benches_objs);
