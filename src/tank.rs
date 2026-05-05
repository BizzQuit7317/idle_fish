use serde::{Serialize, Deserialize}; 

use crate::fish;
use crate::traits;

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
            temprature: 20.0,
            ph: 6.5,
            gh: 7.0,
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
    pub ideal_parameters: fish::Tolerances,

    //define fish stats
    pub max_fish: u8,
    pub fish: Vec<fish::Fish>,
}

impl Tank {
    pub fn new() -> Tank {
        Tank{
            water_parameters: WaterParameters::new(),
            ideal_parameters: fish::Tolerances {
                temprature_range: fish::ParameterRange::new(0.0, 40.0),   // full viable fish range in celsius
                ph_range: fish::ParameterRange::new(0.0, 14.0),           // full pH scale
                gh_range: fish::ParameterRange::new(0.0, 30.0),           // 0 (RO) to very hard
                nitrate_range: fish::ParameterRange::new(0.0, 100.0),     // 0 to dangerously high
                nitrite_range: fish::ParameterRange::new(0.0, 5.0),       // anything above 5 is lethal for most fish
                ammonia_range: fish::ParameterRange::new(0.0, 5.0),       // same as nitrite
            },

            //define fish statsnew
            max_fish: 3,
            fish: vec![
                //fish::Fish::new(),
                //fish::Fish::new(),
                //fish::Fish::new(),
            ],
        }
    }

    pub fn check_fish(&mut self) -> bool {
        let before = self.fish.len();
        self.fish.retain(|fish| fish.alive);
        self.fish.len() != before  // returns true if anything was removed
    }

    pub fn apply_traits(&mut self) {
        for fish in &self.fish {
            for fish_trait in &fish.fish_traits {
                match fish_trait.trait_name {
                    traits::TraitNames::TempratureBoost => self.water_parameters.temprature *= fish_trait.multiplier,
                    traits::TraitNames::AmmoniaBoost => self.water_parameters.ammonia *= fish_trait.multiplier,
                    traits::TraitNames::PHBoost => self.water_parameters.ph *= fish_trait.multiplier,
                    traits::TraitNames::GHBoost => self.water_parameters.gh *= fish_trait.multiplier,
                    traits::TraitNames::NitrateBoost => self.water_parameters.nitrate *= fish_trait.multiplier,
                    traits::TraitNames::NitriteBoost => self.water_parameters.nitrite *= fish_trait.multiplier,
                }
            }
        }
    }
    
    pub fn update_ideal_parameters(&mut self) {
        if self.fish.is_empty() {
            self.ideal_parameters.temprature_range = fish::ParameterRange::new(0.0, 40.0);
            self.ideal_parameters.ph_range = fish::ParameterRange::new(7.0, 7.0);      // neutral
            self.ideal_parameters.gh_range = fish::ParameterRange::new(0.0, 0.0);      // RO = 0 hardness
            self.ideal_parameters.nitrate_range = fish::ParameterRange::new(0.0, 0.0);
            self.ideal_parameters.nitrite_range = fish::ParameterRange::new(0.0, 0.0);
            self.ideal_parameters.ammonia_range = fish::ParameterRange::new(0.0, 0.0);
        } else {
            let mut new_min_temp = f64::MAX;
            let mut new_max_temp = f64::MAX;

            let mut new_min_ph = f64::MAX;
            let mut new_max_ph = f64::MAX;

            let mut new_min_gh = f64::MAX;
            let mut new_max_gh = f64::MAX;

            let mut new_min_nitrate = f64::MAX;
            let mut new_max_nitrate = f64::MAX;

            let mut new_min_nitrite = f64::MAX;
            let mut new_max_nitrite = f64::MAX;

            let mut new_min_ammonia = f64::MAX;
            let mut new_max_ammonia = f64::MAX;

            for fish in &self.fish {
                //println!("[DBG] Tolerances: {:.5?}", fish.tolerances);
                new_min_temp = new_min_temp.min(fish.tolerances.temprature_range.min);
                new_max_temp = new_max_temp.max(fish.tolerances.temprature_range.max);

                new_min_ph = new_min_ph.min(fish.tolerances.ph_range.min);
                new_max_ph = new_max_ph.max(fish.tolerances.ph_range.max);

                new_min_gh = new_min_gh.min(fish.tolerances.gh_range.min);
                new_max_gh = new_max_gh.max(fish.tolerances.gh_range.max);

                new_min_nitrate = new_min_nitrate.min(fish.tolerances.nitrate_range.min);
                new_max_nitrate = new_max_nitrate.max(fish.tolerances.nitrate_range.max);

                new_min_nitrite = new_min_nitrite.min(fish.tolerances.nitrite_range.min);
                new_max_nitrite = new_max_nitrite.max(fish.tolerances.nitrite_range.max);

                new_min_ammonia = new_min_ammonia.min(fish.tolerances.ammonia_range.min);
                new_max_ammonia = new_max_ammonia.max(fish.tolerances.ammonia_range.max);

            }

            self.ideal_parameters.temprature_range.min = new_min_temp;
            self.ideal_parameters.temprature_range.max = new_max_temp;

            self.ideal_parameters.ph_range.min = new_min_ph;
            self.ideal_parameters.ph_range.max = new_max_ph;

            self.ideal_parameters.gh_range.min = new_min_gh;
            self.ideal_parameters.gh_range.max = new_max_gh;

            self.ideal_parameters.nitrate_range.min = new_min_nitrate;
            self.ideal_parameters.nitrate_range.max = new_max_nitrate;

            self.ideal_parameters.nitrite_range.min = new_min_nitrite;
            self.ideal_parameters.nitrite_range.max = new_max_nitrite;

            self.ideal_parameters.ammonia_range.min = new_min_ammonia;
            self.ideal_parameters.ammonia_range.max = new_max_ammonia;
        }
        
    }
}