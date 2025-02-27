use std::vec;

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
}

pub struct ImageRGB {
    data: Vec<Vec<(u8, u8, u8)>>,
}

impl ImageRGB {
    pub fn new(w: u32, h: u32) -> Self {
        let line = vec![(0, 0, 0); w as usize];
        ImageRGB {
            data: vec![line; h as usize],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: (u8, u8, u8)) {
        self.data[x][y] = value;
    }
}

pub fn is_in_circle(pos: &(i64, i64), r: &i64, x: &i64, y: &i64) -> bool {
    if (x - pos.0) * (x - pos.0) + (y - pos.1) * (y - pos.1) < (*r * *r) {
        true
    } else {
        false
    }
}
