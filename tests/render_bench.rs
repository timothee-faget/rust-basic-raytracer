use ray_tracer::mods::parser::Parser;
use std::time::Instant;

#[test]
fn benchmark_render_scene() {
    let mut parser = Parser::new("scenes/test_1.rtp");
    let mut scene_parser = parser.parse_scene();

    let start = Instant::now();
    scene_parser.render();
    let duration = start.elapsed();

    // let _ = scene_parser.save_image("outputs/test_1");

    println!("RenderTimeMs: {}", duration.as_millis());
}

#[test]
fn benchmark_parse_scene() {
    let start = Instant::now();
    let mut parser = Parser::new("scenes/test_1.rtp");
    let _scene_parser = parser.parse_scene();

    let duration = start.elapsed();

    println!("RenderTimeMs: {}", duration.as_millis());
}
