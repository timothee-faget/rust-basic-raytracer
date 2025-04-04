use std::error::Error;
use std::io::prelude::*;
use std::{fs::File, vec};

pub struct ImageWB {
    data: Vec<Vec<u8>>,
}

impl ImageWB {
    pub fn new(w: u32, h: u32) -> Self {
        let line = vec![0; w as usize];
        ImageWB {
            data: vec![line; h as usize],
        }
    }

    pub fn print(&self) {
        for line in self.data.iter() {
            for el in line {
                print!("{}", el)
            }
            println!("");
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        self.data[y][x] = value;
    }

    pub fn save_as_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(format!("{}.pbm", filename))?;
        let header = format!("P1\n{} {}\n", self.data[0].len(), self.data.len());

        file.write_all(header.as_bytes())?;
        for line in self.data.iter() {
            for el in line {
                let el_str = format!("{}", el);
                file.write_all(el_str.as_bytes())?;
            }
            file.write_all("\n".as_bytes())?;
        }
        Ok(())
    }
}

pub struct ImageRGB {
    data: Vec<Vec<(u8, u8, u8)>>,
}

impl ImageRGB {
    pub fn new(w: u32, h: u32) -> Self {
        let line = vec![(255, 255, 255); w as usize];
        ImageRGB {
            data: vec![line; h as usize],
        }
    }

    pub fn get_width(&self) -> usize {
        self.data[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.data.len()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: (u8, u8, u8)) {
        self.data[y][x] = value;
    }

    pub fn save_as_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        // println!("Saving RGB Image to ppm file");
        let mut file = File::create(format!("{}.ppm", filename))?;
        let header = format!("P3\n{} {}\n255\n", self.data[0].len(), self.data.len());

        file.write_all(header.as_bytes())?;

        let mut content = String::new();
        for line in self.data.iter() {
            for el in line {
                content += format!("{} {} {}\n", el.0, el.1, el.2).as_str();
            }
        }
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
