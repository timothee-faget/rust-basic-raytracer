use rbpt::render_scene;

fn main() {
    render_scene(
        "scenes/teapot_2.rtp",
        "example_teapot_2",
        20,
        10,
        (480, 360),
    )
    .unwrap();
}
