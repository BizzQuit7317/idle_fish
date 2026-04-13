use crate::tank::Tank;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TraitNames {
    TempratureBoost,
    AmmoniaBoost,
    PHBoost,
    GHBoost,
    NitrateBoost,
    NitriteBoost,
}

impl TraitNames {
    pub fn apply(&self, tank: &mut Tank, multiplier: f64) {
        match self {
            TraitNames::TempratureBoost => tank.water_parameters.temprature *= multiplier,
            TraitNames::AmmoniaBoost => tank.water_parameters.ammonia *= multiplier,
            TraitNames::PHBoost => tank.water_parameters.ph *= multiplier,
            TraitNames::GHBoost => tank.water_parameters.gh *= multiplier,
            TraitNames::NitrateBoost => tank.water_parameters.nitrate *= multiplier,
            TraitNames::NitriteBoost => tank.water_parameters.nitrite *= multiplier,
        }
    }

    pub fn from_str(s: &str) -> TraitNames {
        match s {
            "TempratureBoost" => TraitNames::TempratureBoost,
            "AmmoniaBoost" => TraitNames::AmmoniaBoost,
            "PHBoost" => TraitNames::PHBoost,
            "GHBoost" => TraitNames::GHBoost,
            "NitrateBoost" => TraitNames::NitrateBoost,
            "NitriteBoost" => TraitNames::NitriteBoost,
            _ => panic!("Unknown trait: {}", s),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trait {
    pub trait_name: TraitNames,
    pub multiplier: f64,
    pub weight: f64, //weighted averages for breeding and passing traits on
}

impl Trait {
    pub fn new(trait_name: TraitNames, multiplier: f64, weight: f64) -> Trait {
        Trait {
            trait_name:trait_name,
            multiplier:multiplier,
            weight:weight,
        }
    }
}