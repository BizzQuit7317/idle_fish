use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    pub on: bool, //A bool to keep track weather the light is switched on or off
}

impl Light {
    pub fn new() -> Light {
        Light {
            on: false, //Start with the lights off
        }
    }
}