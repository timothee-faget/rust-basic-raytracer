use rbpt::render_scene;

fn main() {
    render_scene("scenes/teapot.rtp", "example_teapot", 20, 10, (480, 360)).unwrap();
}
