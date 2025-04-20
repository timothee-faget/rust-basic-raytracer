pub mod mods;
use std::{error::Error, time::Instant};

use mods::parser::Parser;

pub fn basic_ray_tracing() -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::build("scenes/rendu_2.rtp")?;
    let mut scene_parser = parser.parse_scene();
    let start = Instant::now();
    scene_parser.render_bounces();
    let duration = start.elapsed().as_millis();
    println!("Time : {duration}ms");
    scene_parser.save_image("outputs/rendu_2test")?;
    Ok(())
}
