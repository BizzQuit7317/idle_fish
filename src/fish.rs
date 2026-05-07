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
    pub min_tank_size: u8,
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
            min_tank_size: species.min_tank_size,
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

    pub fn get_prestige(&self) -> f64 {
        /*
            Needs to modify the output prestige of a fish based on its status
        */
        match &self.status {
            FishStatus::Thriving => {
                return 10.0
            },
            FishStatus::Healthy => {
                return 2.0
            },
            FishStatus::Neatural => {
                return 1.0
            },
            FishStatus::Sick => {
                return 0.5
            },
            FishStatus::Dead => {
                return 0.0
            },         

        }
    }

    /*
        This calculation needs to check if each paramter of the water is within this fish's range
        if its not then we need to take the differenec and multiply by the WELLNESS_PENALTY_SEVERITY
        the calculation starts at 60.0 points to make thriving a special and healthy hard to get to.
        This should be changeable as needed to otweak the formula
    */
    pub fn calculate_wellness(&mut self, water: &tank::WaterParameters) {
    let temp_score    = self.parameter_score(water.temprature, &self.tolerances.temprature_range);
    let ph_score      = self.parameter_score(water.ph,         &self.tolerances.ph_range);
    let gh_score      = self.parameter_score(water.gh,         &self.tolerances.gh_range);
    let nitrate_score = self.parameter_score(water.nitrate,    &self.tolerances.nitrate_range);
    let nitrite_score = self.parameter_score(water.nitrite,    &self.tolerances.nitrite_range);
    let ammonia_score = self.parameter_score(water.ammonia,    &self.tolerances.ammonia_range);

    let average_param_score = ( temp_score +  ph_score + gh_score + nitrate_score + nitrite_score + ammonia_score ) / 6.0;//6 is just the number of parameters

    //println!("[DBG]scores temp_score{} ph_score{} gh_score{} nitrate_score{} nitrite_score{} ammonia_score{} average_param_score{}", temp_score, ph_score, gh_score, nitrate_score, nitrite_score, ammonia_score, average_param_score);

    self.wellness = average_param_score; // Need to also mdify by hunger so wellness drops at low hunger
}

pub fn parameter_score(&self, parameter_value: f64, range: &ParameterRange) -> f64 {
    let mid_point      = (range.min + range.max) / 2.0;
    let half_range_size = (range.max - range.min) / 2.0;

    let deviation = (parameter_value - mid_point).abs() / half_range_size;

    if deviation <= 1.0 {
        60.0 - (deviation * 20.0)
    } else {
        (40.0 - ((deviation - 1.0) * 40.0)).max(0.0)
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
        let drain = match self.tier {
            FishTier::Nano         => constants::HUNGER_DRAIN_NANO,
            FishTier::Community    => constants::HUNGER_DRAIN_COMMUNITY,
            FishTier::Tropical     => constants::HUNGER_DRAIN_TROPICAL,
            FishTier::Predator     => constants::HUNGER_DRAIN_PREDATOR,
            FishTier::RiverMonster => constants::HUNGER_DRAIN_RIVER_MONSTER,
        };

        self.hunger = (self.hunger - drain).max(0.0); // clamp at 0, never go negative
    }

    pub fn eat(&mut self, food_level: f64) { 
        //should alsso be affected by tier a smaller fih should get more per each food than a larger one
        let restore = constants::BASE_FOOD_RESTORE * food_level.powf(1.5);
        self.hunger = (self.hunger + restore);
    }

    pub fn increase_age(&mut self) {
        self.age += 1 //Just needs to increase per tick controlled by max age range
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