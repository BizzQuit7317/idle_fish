use serde::{Serialize, Deserialize};

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
}