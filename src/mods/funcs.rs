pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let delta = b * b - 4.0 * a * c;
    if delta < 0.0 {
        None
    } else {
        let delta_sqrt = delta.sqrt();
        let mut t1 = (-b + delta_sqrt) / (2.0 * a);
        let mut t2 = (-b - delta_sqrt) / (2.0 * a);
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }
        Some((t1, t2))
    }
}
