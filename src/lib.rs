pub mod mods;
use mods::{funcs, img};

pub fn create_basic_text_image() {
    let w: u32 = 1000;
    let h: u32 = 1000;

    let r: i64 = 150;
    let pos: (i64, i64) = (300, 500);

    let mut img_rgb = img::ImageRGB::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let x_i = x as i64;
            let y_i = y as i64;
            if funcs::is_in_circle(&pos, &r, &x_i, &y_i) {
                let value = funcs::gradient_from_center(&pos, &r, &x_i, &y_i);
                img_rgb.set_pixel(x as usize, y as usize, (value, 0, value));
            }
        }
    }
    img_rgb.save_as_file("image_RGB").unwrap();
}
