use ray_tracer::basic_ray_tracing;

fn main() {
    if let Err(e) = basic_ray_tracing() {
        println!("Erreur : {e}");
    }
}
