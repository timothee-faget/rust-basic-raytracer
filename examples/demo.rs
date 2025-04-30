use rbpt::render_scene;

fn main() {
    render_scene("scenes/demo.rtp", "example_demo", 20, 10, (480, 360)).unwrap();
}
