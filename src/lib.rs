pub mod mods;
use std::error::Error;

use mods::parser::Parser;

/// Render scene
///    scene_file: .rtp file describing the scene
///    image_file: file to save image to
///    render_iterations: number of render iterations to average
///    max_bounces: number of max ray bounces
///    resolution: (width, height) of the rendered image
pub fn render_scene(
    scene_file: &str,
    image_file: &str,
    render_iterations: usize,
    max_bounces: u32,
    resolution: (u32, u32),
) -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::build(scene_file)?;
    let mut scene = parser.parse_scene();

    scene.render(render_iterations, max_bounces, resolution);
    scene.save_image(image_file)?;

    Ok(())
}
