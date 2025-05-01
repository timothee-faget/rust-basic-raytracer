use criterion::{criterion_group, criterion_main, Criterion};
use rbpt::mods::{
    color::{lerp_color, ColorRBG},
    funcs::{reflect, solve_quadratic},
    position::{lerp, Quat, Vect3},
    random::LCG,
};

pub fn bench_vect3(c: &mut Criterion) {
    let vect_test_1 = Vect3::new(1.0, 2.0, 3.0);
    let vect_test_2 = Vect3::new(-1.0, 3.0, -4.0);

    c.bench_function("norm", |b| b.iter(|| vect_test_1.norm()));
    c.bench_function("normalize", |b| b.iter(|| vect_test_1.normalize()));
    c.bench_function("add", |b| b.iter(|| vect_test_1 + vect_test_2));
    c.bench_function("dot_old", |b| b.iter(|| vect_test_1 * vect_test_2));
    c.bench_function("dot_new", |b| b.iter(|| vect_test_1.dot(&vect_test_2)));
    c.bench_function("prod", |b| b.iter(|| vect_test_1.prod(vect_test_2)));
    c.bench_function("new", |b| b.iter(|| Vect3::new(1.0, 1.0, 1.0)));
    c.bench_function("lerp", |b| b.iter(|| lerp(vect_test_1, vect_test_2, 0.5)));
}

pub fn bench_quat(c: &mut Criterion) {
    let quat_test_1 = Quat::new(1.0, Vect3::new(0.0, 1.0, 0.0));

    c.bench_function("identity", |b| b.iter(|| Quat::identity()));
    c.bench_function("from_axis_angle", |b| {
        b.iter(|| Quat::from_axis_angle(Vect3::UP, 1.0))
    });
    c.bench_function("from_axis_angle_deg", |b| {
        b.iter(|| Quat::from_axis_angle_deg(Vect3::UP, 90.0))
    });
    c.bench_function("normalize", |b| b.iter(|| quat_test_1.normalize()));
    c.bench_function("conjugate", |b| b.iter(|| quat_test_1.conjugate()));
    c.bench_function("rotate", |b| {
        b.iter(|| quat_test_1.rotate(Vect3::new(1.0, 0.0, 1.0)))
    });
}

pub fn bench_colors(c: &mut Criterion) {
    let color_test_1 = ColorRBG::new(0.5, 0.0, 1.0);
    let color_test_2 = ColorRBG::new(1.0, 0.0, 0.7);

    c.bench_function("get_value", |b| b.iter(|| color_test_1.get_value()));
    c.bench_function("rgb", |b| b.iter(|| color_test_1.rgb()));
    c.bench_function("max_component", |b| b.iter(|| color_test_1.max_component()));
    c.bench_function("lerp_color", |b| {
        b.iter(|| lerp_color(color_test_1, color_test_2, 0.5))
    });
}

pub fn bench_funcs(c: &mut Criterion) {
    c.bench_function("solve_quadratics_1", |b| {
        b.iter(|| solve_quadratic(1.0, 0.0, 1.0))
    });
    c.bench_function("solve_quadratics_2", |b| {
        b.iter(|| solve_quadratic(1.0, 0.0, 0.0))
    });
    c.bench_function("solve_quadratics_3", |b| {
        b.iter(|| solve_quadratic(1.0, 0.0, -1.0))
    });
    let vect_test_1 = Vect3::new(0.5, 0.5, 0.0);
    let vect_test_2 = Vect3::new(0.0, 1.0, 0.0);
    c.bench_function("reflect", |b| b.iter(|| reflect(vect_test_1, vect_test_2)));
}

pub fn bench_lcg(c: &mut Criterion) {
    let mut lcg = LCG::new(123456789);
    let vect_test = Vect3::UP;
    c.bench_function("next_f64", |b| b.iter(|| lcg.next_f64()));
    c.bench_function("next_vect3", |b| b.iter(|| lcg.next_vect3()));
    c.bench_function("next_normal_vect3", |b| {
        b.iter(|| lcg.next_normal_vect3(vect_test))
    });
}

criterion_group!(
    benches_comps,
    bench_vect3,
    bench_quat,
    bench_colors,
    bench_funcs,
    bench_lcg
);
criterion_main!(benches_comps);
