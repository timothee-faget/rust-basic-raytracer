pub mod mods;
use mods::img;

pub fn create_basic_text_image() {
    let w: u32 = 64;
    let h: u32 = 48;

    let r: i64 = 10;
    let pos: (i64, i64) = (32, 24);

    let mut img = img::ImageWB::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let x_i = x as i64;
            let y_i = y as i64;
            if img::is_in_circle(&pos, &r, &x_i, &y_i) {
                img.set_pixel(x as usize, y as usize, 1);
            }
        }
    }

    img.print();
}
