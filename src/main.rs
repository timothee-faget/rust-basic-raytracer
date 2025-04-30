use clap::{Arg, Command};

use rbpt::{
    mods::config::{parse_quality, parse_resolution},
    render_scene,
};

fn main() {
    let matches = Command::new("Mon Programme")
        .version("1.0")
        .author("Timothee FAGET")
        .about("Basic Ray tracer written in Rust")
        .arg(
            Arg::new("scene_file")
                .help(".rtp file describing the scene")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("image_file")
                .help("file to save image to (with '.ppm' extension or not)")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("quality")
                .short('q')
                .long("quality")
                .value_name("QUALITY")
                .help("Quality: (H, h, m, l, L), 1 or 2 numbers")
                .num_args(1..=2)
                .required(false),
        )
        .arg(
            Arg::new("resolution")
                .short('r')
                .long("resolution")
                .value_name("Resolution")
                .help("Resolution: (H, h, m, l, L) or 2 numbers")
                .num_args(1..=2)
                .required(false),
        )
        .get_matches();

    let scene_file = matches.get_one::<String>("scene_file").unwrap();
    let image_file = matches.get_one::<String>("image_file").unwrap();
    let quality = parse_quality(&matches);
    let resolution = parse_resolution(&matches);

    if let Err(e) = render_scene(scene_file, image_file, quality.0, quality.1, resolution) {
        println!("Erreur : {e}");
    }
}
