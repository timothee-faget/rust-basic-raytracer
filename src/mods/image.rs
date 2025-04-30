// Image stuff

use std::{error::Error, fs::File, io::prelude::*};

use console::style;

/// Image implementation
#[derive(Clone)]
pub struct ImageRGB {
    data: Vec<Vec<(u8, u8, u8)>>,
}

impl ImageRGB {
    /// New Image constructor
    pub fn new(w: u32, h: u32) -> Self {
        let line = vec![(255, 255, 255); w as usize];
        ImageRGB {
            data: vec![line; h as usize],
        }
    }

    /// Get width
    #[inline]
    pub fn get_width(&self) -> usize {
        self.data[0].len()
    }

    /// Get height
    #[inline]
    pub fn get_height(&self) -> usize {
        self.data.len()
    }

    /// Get number of pixels
    #[inline]
    pub fn get_pixel_count(&self) -> usize {
        self.data.len() * self.data[0].len()
    }

    /// Set a pixel's color
    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, value: (u8, u8, u8)) {
        self.data[y][x] = value;
    }

    /// Save as ppm file (not optimal...)
    pub fn save_as_ppm(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut filename: String = filename.to_string();
        if !filename.ends_with(".ppm") {
            filename = format!("{}.ppm", filename);
        }
        print_save_info(&filename);
        let mut file = File::create(filename)?;
        let header = format!("P3\n{} {}\n255\n", self.data[0].len(), self.data.len());

        file.write_all(header.as_bytes())?;

        let mut content = String::new();
        for line in self.data.iter() {
            for el in line {
                content += format!("{} {} {}\n", el.0, el.1, el.2).as_str();
            }
        }
        file.write_all(content.as_bytes())?;
        println!("      Saved image to ppm file");
        Ok(())
    }
}

/// Prints saving information
fn print_save_info(filename: &String) {
    println!(
        "{} Saving image to : {}",
        style("[3/3]").bold().green(),
        style(filename).italic().dim()
    );
}

#[cfg(test)]
mod tests_image {
    use super::ImageRGB;

    #[test]
    fn getters() {
        let image = ImageRGB::new(10, 5);

        assert_eq!(image.get_width(), 10);
        assert_eq!(image.get_height(), 5);
        assert_eq!(image.get_pixel_count(), 50);
    }

    #[test]
    fn set() {
        let mut image = ImageRGB::new(10, 5);
        image.set_pixel(0, 0, (1, 1, 1));

        assert_eq!(image.data[0][0], (1, 1, 1));
    }

    #[test]
    fn save() {
        let mut image = ImageRGB::new(10, 5);

        assert!(image.save_as_ppm("tests_image_save").is_ok())
    }
}
