use clap::ArgMatches;
use std::str::FromStr;

/// Structure for quality choice
pub struct RenderQuality {
    ri: usize,
    mb: u32,
}

impl RenderQuality {
    pub const ULTRA_HIGH: Self = RenderQuality { ri: 300, mb: 40 };
    pub const HIGH: Self = RenderQuality { ri: 150, mb: 30 };
    pub const MEDIUM: Self = RenderQuality { ri: 50, mb: 13 };
    pub const LOW: Self = RenderQuality { ri: 20, mb: 8 };
    pub const ULTRA_LOW: Self = RenderQuality { ri: 5, mb: 5 };

    pub fn new(mb: u32, ri: usize) -> Self {
        Self { ri, mb }
    }

    pub fn mb(&self) -> u32 {
        self.mb
    }

    pub fn ri(&self) -> usize {
        self.ri
    }
}

/// Structure for resolution choice
pub struct RenderResolution {
    r: (u32, u32),
}

impl RenderResolution {
    pub const ULTRA_HIGH: Self = RenderResolution { r: (3840, 2160) };
    pub const HIGH: Self = RenderResolution { r: (1920, 1080) };
    pub const MEDIUM: Self = RenderResolution { r: (1280, 720) };
    pub const LOW: Self = RenderResolution { r: (720, 480) };
    pub const ULTRA_LOW: Self = RenderResolution { r: (360, 240) };

    pub fn new(w: u32, h: u32) -> Self {
        Self { r: (w, h) }
    }

    pub fn rs(&self) -> (u32, u32) {
        self.r
    }

    pub fn w(&self) -> u32 {
        self.r.0
    }

    pub fn h(&self) -> u32 {
        self.r.1
    }
}

pub fn parse_quality(matches: &ArgMatches) -> (usize, u32) {
    let mut quality = RenderQuality::MEDIUM;

    if let Some(qualities) = matches.get_many::<String>("quality") {
        let qualities: Vec<String> = qualities.cloned().collect();

        if qualities.len() == 1
            && qualities[0].len() == 1
            && qualities[0].chars().next().unwrap().is_alphabetic()
        {
            match qualities[0].as_str() {
                "H" => quality = RenderQuality::ULTRA_HIGH,
                "h" => quality = RenderQuality::HIGH,
                "m" => quality = RenderQuality::MEDIUM,
                "l" => quality = RenderQuality::LOW,
                "L" => quality = RenderQuality::ULTRA_LOW,
                _ => quality = RenderQuality::MEDIUM,
            }
        } else if qualities.len() == 1 {
            match usize::from_str(&qualities[0]) {
                Ok(num) => {
                    quality = RenderQuality::new(num.min(20) as u32, num);
                }
                _ => {
                    eprintln!("wrong quality argument");
                    std::process::exit(1);
                }
            }
        } else if qualities.len() == 2 {
            if let (Ok(ri), Ok(mb)) = (usize::from_str(&qualities[0]), u32::from_str(&qualities[1]))
            {
                quality = RenderQuality::new(mb, ri);
            }
        } else {
            eprintln!("wrong quality argument");
            std::process::exit(1);
        }
    } else {
        quality = RenderQuality::MEDIUM;
    }

    (quality.ri(), quality.mb())
}

pub fn parse_resolution(matches: &ArgMatches) -> (u32, u32) {
    let resolution: RenderResolution;
    if let Some(resolutions) = matches.get_many::<String>("resolution") {
        let resolutions: Vec<String> = resolutions.cloned().collect();

        if resolutions.len() == 1
            && resolutions[0].len() == 1
            && resolutions[0].chars().next().unwrap().is_alphabetic()
        {
            // TODO
            match resolutions[0].as_str() {
                "H" => resolution = RenderResolution::ULTRA_HIGH,
                "h" => resolution = RenderResolution::HIGH,
                "m" => resolution = RenderResolution::MEDIUM,
                "l" => resolution = RenderResolution::LOW,
                "L" => resolution = RenderResolution::ULTRA_LOW,
                _ => resolution = RenderResolution::MEDIUM,
            }
        } else if resolutions.len() == 2 {
            match (
                u32::from_str(&resolutions[0]),
                u32::from_str(&resolutions[1]),
            ) {
                (Ok(w), Ok(h)) => {
                    resolution = RenderResolution::new(w, h);
                }
                _ => {
                    eprintln!("wrong resolution argument");
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("wrong resolution argument");
            std::process::exit(1);
        }
    } else {
        resolution = RenderResolution::MEDIUM;
    }

    resolution.rs()
}
