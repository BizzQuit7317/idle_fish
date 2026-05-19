use serde::{Serialize, Deserialize};

use crate::components;

#[derive(Debug, Serialize, Deserialize)]
pub struct Algea {
    pub light_levels: f64,
    pub nutrient_levels: f64,
    pub surface_area: f64,
    pub coverge: f64,
}

impl Algea {
    pub fn new() -> Algea {
        Algea {
            light_levels: 0.0,
            nutrient_levels: 0.0,
            surface_area: 0.0,
            coverge: 0.0,
        }
    }

    pub fn grow(&mut self, lighting: components::lights::Light) {
        match lighting.spectrum {
            components::lights::Spectrum::Full => {
                self.light_levels += lighting.intensity //Flat grow rate no extra improvements
            },
            components::lights::Spectrum::Grow => {
                self.light_levels += lighting.intensity * 1.5 //some improvments to growing
            },
            components::lights::Spectrum::Reef => {
                self.light_levels += lighting.intensity * 2.0 //grows twice as fast
            },
            components::lights::Spectrum::Moonlight => {
                self.light_levels += lighting.intensity * 0.5 //grows at half the rate
            },
        }
    }
}