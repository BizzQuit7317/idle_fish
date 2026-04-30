use serde::{Serialize, Deserialize}; 

use crate::fish;

#[derive(Debug, Serialize, Deserialize)]
pub enum WaterParameter {
    temprature,
    ph,
    gh,
    nitrate,
    nitrite,
    ammonia,
}

impl WaterParameter {
    pub fn from_str(s: &str) -> WaterParameter {
        match s {
            "temperature" => WaterParameter::temprature,
            "ph" => WaterParameter::ph,
            "gh" => WaterParameter::gh,
            "nitrate" => WaterParameter::nitrate,
            "nitrite" => WaterParameter::nitrite,
            "ammonia" => WaterParameter::ammonia,
            _ => panic!("Unknown water parameter: {}", s), // panic here is fine, means your json is wrong
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterParameters {
    pub temprature: f64,
    pub ph: f64,
    pub gh: f64,
    pub nitrate: f64,
    pub nitrite: f64,
    pub ammonia: f64,
}

impl WaterParameters {
    pub fn new() -> WaterParameters {
        /*
            Should be set to RO water by default on a new instance
        */
        WaterParameters {
            temprature: 25.0,
            ph: 6.5,
            gh: 0.0,
            nitrate: 0.0,
            nitrite: 0.0,
            ammonia: 0.0,
        }
    }

    pub fn apply_changes(&mut self, parameter: &WaterParameter, value: f64) {
        match parameter {
            WaterParameter::temprature => self.temprature -= value,
            WaterParameter::ph => self.ph += value,
            WaterParameter::gh => self.gh += value,
            WaterParameter::nitrate => self.nitrate += value,
            WaterParameter::nitrite => self.nitrite += value,
            WaterParameter::ammonia => self.ammonia += value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tank {
    //define the water parameterss
    pub water_parameters: WaterParameters,

    //define fish stats
    pub max_fish: u8,
    pub fish: Vec<fish::Fish>,
}

impl Tank {
    pub fn new() -> Tank {
        Tank{
            water_parameters: WaterParameters::new(),

            //define fish statsnew
            max_fish: 3,
            fish: vec![
                //fish::Fish::new(),
                //fish::Fish::new(),
                //fish::Fish::new(),
            ],
        }
    }

    pub fn check_fish(&mut self) {
        self.fish.retain(|fish| fish.alive);
    }
}