use super::color::ColorRBG;

/// Material implementation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: ColorRBG,
    pub emission_color: ColorRBG,
    pub specular_color: ColorRBG,
    pub emission_strengh: f64,
    pub smoothness: f64,
    pub specular_prob: f64,
}

impl Material {
    /// New Material constructor
    pub fn new(
        color: ColorRBG,
        emission_color: ColorRBG,
        specular_color: ColorRBG,
        emission_strengh: f64,
        smoothness: f64,
        specular_prob: f64,
    ) -> Self {
        Self {
            color,
            emission_color,
            specular_color,
            emission_strengh,
            smoothness,
            specular_prob,
        }
    }

    /// Get emitted light
    #[inline]
    pub fn get_emited_light(&self) -> ColorRBG {
        self.emission_strengh.min(1.0) * self.emission_color
    }
}

/// Default Material
impl Default for Material {
    fn default() -> Self {
        Material {
            color: ColorRBG::WHITE,
            emission_color: ColorRBG::WHITE,
            specular_color: ColorRBG::WHITE,
            emission_strengh: 0.0,
            smoothness: 0.5,
            specular_prob: 0.5,
        }
    }
}

#[cfg(test)]
mod material_tests {
    use crate::mods::color::ColorRBG;

    use super::Material;

    #[test]
    fn emitted_light() {
        let mat = Material::new(
            ColorRBG::WHITE,
            ColorRBG::WHITE,
            ColorRBG::WHITE,
            1.0,
            1.0,
            1.0,
        );

        assert_eq!(mat.get_emited_light(), ColorRBG::WHITE);
        assert_eq!(Material::default().get_emited_light(), ColorRBG::BLACK);
    }
}
