use rbpt::render_scene;

fn main() {
    render_scene("scenes/monkey.rtp", "example_monkey", 100, 15, (1280, 720)).unwrap();
}
