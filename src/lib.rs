pub mod mods;
use mods::parser::Parser;

pub fn basic_ray_tracing() {
    let mut parser = Parser::new("scenes/example.rtp");
    let mut scene_parser = parser.parse_scene();
    scene_parser.render();
    let _ = scene_parser.save_image("outputs/test_parser_4");
}
