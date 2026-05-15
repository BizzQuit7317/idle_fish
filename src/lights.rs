use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Spectrum {
    /*
        Not a linear, higher is better, required for differnt things
    */
    Full, //Regular day light
    Grow, //Best lighting for plants
    Reef, //More Intense with high UV
    Moonlight, //A blue light option
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Light {
    pub on: bool, //A bool to keep track weather the light is switched on or off
    pub max_toggles: u8, //A small number t track hw many times the player can turn the lights on and off
    pub toggles_used: u8, //Track how many timess the sswitch has been used
    pub intensity: f64, //The  number used to dictate how much light energy is pssed to other functions
    pub spectrum: Spectrum,
}

impl Light {
    pub fn new() -> Light {
        Light {
            on: false, //Start with the lights off
            max_toggles: 3, //Change lter, just the initial number of toggles
            toggles_used: 0,
            intensity: 1.0, //NEED TO ALTER/BALANCE!!!
            spectrum: Spectrum::Full,
        }
    }
}