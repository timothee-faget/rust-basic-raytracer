pub mod mods;
use std::time::Instant;

use mods::parser::Parser;

pub fn basic_ray_tracing() {
    let mut parser = Parser::new("scenes/rendu_1.rtp");
    let mut scene_parser = parser.parse_scene();
    let start = Instant::now();
    scene_parser.render_bounces();
    let duration = start.elapsed().as_millis();
    println!("Time : {duration}ms");
    let _ = scene_parser.save_image("outputs/rendu_1");
}
