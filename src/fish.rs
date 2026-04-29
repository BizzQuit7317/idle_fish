use crate::traits;
use crate::tank;
use crate::constants;
use crate::registry;
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Moddifier {
    pub parameter: tank::WaterParameter,
    pub moddifier: f64, //this number will usually be reduced as a cost of keeping the fish
}

impl Moddifier {
    pub fn new(parameter: tank::WaterParameter, moddifier: f64) -> Self {
        Moddifier { 
            parameter, 
            moddifier, //this number will usually be reduced as a cost of keeping the fish
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterRange {
    pub min: f64,
    pub max: f64,
}

impl ParameterRange {
    pub fn new(min: f64, max: f64) -> ParameterRange {
        ParameterRange {
            min: min,
            max: max,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tolerances {
    pub temprature_range: ParameterRange,
    pub ph_range: ParameterRange,
    pub gh_range: ParameterRange,
    pub nitrate_range: ParameterRange,
    pub nitrite_range: ParameterRange,
    pub ammonia_range: ParameterRange,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FishTier {
    Nano,
    Community,
    Tropical,
    Predator,
    RiverMonster,
}

impl FishTier {
    pub fn from_str(s: &str) -> FishTier {
        match s {
            "Nano" => FishTier::Nano,
            "Community" => FishTier::Community,
            "Tropical" => FishTier::Tropical,
            "Predator" => FishTier::Predator,
            "RiverMonster" => FishTier::RiverMonster,
            _ => FishTier::Nano, // default fallback for now
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FishStatus {
    Thriving,
    Healthy,
    Neatural,
    Sick,
    Dead,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fish {
    pub species: String, //Mostly just a display for the users
    pub age: u32,
    pub max_age: u32, //Have this be some random range each fish
    pub alive: bool, //Keep track of alive or dead fish
    pub min_group: u8,
    pub tier: FishTier,
    pub fish_traits: Vec<traits::Trait>,

    pub tolerances: Tolerances,

    pub moddifiers: Vec<Moddifier>,
    pub hunger: f64,
    pub status: FishStatus, //calculated based on w wellness score from the tank parameters
    pub wellness: f64, //the int that will drive the status calculation

    pub base_prestige: f64, //how you earn money in the game
    pub base_cost: f64,
}

impl Fish {
    pub fn new(species: &registry::FishSpecies) -> Fish {
        let mut rng = rand::thread_rng();
        let max_age = rng.gen_range(species.max_age_range.min..=species.max_age_range.max);

        Fish {
            species: species.species.clone(),
            age: 0,
            max_age, //Have this be some random range each fish
            alive: true, //start on true until either health or age hit max
            min_group: species.min_group,
            tier: FishTier::from_str(&species.tier), //keep hardcoded for now convert the enum later
            fish_traits: species.traits.iter().map(|t| {
                traits::Trait::new(
                    traits::TraitNames::from_str(&t.trait_name),
                    t.multiplier,
                    t.weight,
                )
            }).collect(),

            tolerances: Tolerances {
                temprature_range: ParameterRange::new(
                    species.tolerances.temperature_range.min,
                    species.tolerances.temperature_range.max,
                ),
                ph_range: ParameterRange::new(
                    species.tolerances.ph_range.min,
                    species.tolerances.ph_range.max,
                ),
                gh_range: ParameterRange::new(
                    species.tolerances.gh_range.min,
                    species.tolerances.gh_range.max,
                ),
                nitrate_range: ParameterRange::new(
                    species.tolerances.nitrate_range.min,
                    species.tolerances.nitrate_range.max,
                ),
                nitrite_range: ParameterRange::new(
                    species.tolerances.nitrite_range.min,
                    species.tolerances.nitrite_range.max,
                ),
                ammonia_range: ParameterRange::new(
                    species.tolerances.ammonia_range.min,
                    species.tolerances.ammonia_range.max,
                ),
            },

            moddifiers: species.modifiers.iter().map(|m| {
                Moddifier::new(
                    tank::WaterParameter::from_str(&m.parameter),
                    m.modifier,
                )
            }).collect(),
            hunger: 50.0,
            status: FishStatus::Neatural, //calculated based on w wellness score from the tank parameters
            wellness: 60.0, //the int that will drive the status calculation

            base_prestige: species.base_prestige,

            base_cost: species.base_cost,
            
        }
    }

    /*
        This calculation needs to check if each paramter of the water is within this fish's range
        if its not then we need to take the differenec and multiply by the WELLNESS_PENALTY_SEVERITY
        the calculation starts at 60.0 points to make thriving a special and healthy hard to get to.
        This should be changeable as needed to otweak the formula
    */
    pub fn calculate_wellness(&mut self, water: &tank::WaterParameters) {
        let mut score = 60.0; //Starts the fish at neutral

        //println!("[DBG] Initial Score {}", score);

        score -= self.calculate_parameter_penalty(water.temprature, &self.tolerances.temprature_range);
        //println!("[DBG] Post temprature Score {}", score);
        score -= self.calculate_parameter_penalty(water.ph, &self.tolerances.ph_range);
        //println!("[DBG] Post ph Score {}", score);
        score -= self.calculate_parameter_penalty(water.gh, &self.tolerances.gh_range);
        //println!("[DBG] Post gh Score {}", score);
        score -= self.calculate_parameter_penalty(water.nitrate, &self.tolerances.nitrate_range);
        //println!("[DBG] Post nitrate Score {}", score);
        score -= self.calculate_parameter_penalty(water.nitrite, &self.tolerances.nitrite_range);
        //println!("[DBG] Post nitrite Score {}", score);
        score -= self.calculate_parameter_penalty(water.ammonia, &self.tolerances.ammonia_range);
        //println!("[DBG] Post ammonia Score {}", score);    

        self.wellness = score;
    }

    pub fn calculate_parameter_penalty(&self, value: f64, range: &ParameterRange) -> f64 {
        if value < range.min {
            (range.min - value) * constants::WELLNESS_PENALTY_SEVERITY
        } else if value > range.max {
            (value - range.max) * constants::WELLNESS_PENALTY_SEVERITY
        } else {
            0.0
        }
    }

    pub fn status_check(&mut self) {
        self.status = match self.wellness {
            s if s >= constants::WELLNESS_THRIVING => FishStatus::Thriving,
            s if s >= constants::WELLNESS_HEALTHY  => FishStatus::Healthy,
            s if s >= constants::WELLNESS_NEUTRAL  => FishStatus::Neatural, // Fixed spelling from your enum
            s if s >= constants::WELLNESS_SICK     => FishStatus::Sick,
            _                                      => FishStatus::Dead,
        }
    }

    pub fn calculate_hunger(&mut self) {
        /*
            Need some formula here to decrease hunger by a factor 
            related to the wellness
        */
        self.hunger -= 1.0
    }

    pub fn eat(&mut self) {
        /*
            Need to alter this per fish so its not a flat value
        */

        self.hunger += 2.0
    }

    pub fn increase_age(&mut self) {
        /*
            Need to make this a formula aswell to have age scale differently for differnt species
            do not want age to be a flat rate
        */
        self.age += 1
    }


    pub fn alive_check(&mut self) {
        if self.hunger <= 0.0 {
            self.alive = false
        }

        if self.age >= self.max_age {
            self.alive = false
        }

        if self.status == FishStatus::Dead {
            self.alive = false
        }
    }
}