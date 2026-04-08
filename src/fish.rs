use crate::traits;
use crate::tank;
use crate::constants;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Tolerances {
    pub temprature_range: ParameterRange,
    pub ph_range: ParameterRange,
    pub gh_range: ParameterRange,
    pub nitrate_range: ParameterRange,
    pub nitrite_range: ParameterRange,
    pub ammonia_range: ParameterRange,
}

impl Tolerances {
    pub fn new() -> Tolerances {
        Tolerances {
            temprature_range: ParameterRange::new(22.0, 27.0),
            ph_range: ParameterRange::new(22.0, 27.0),
            gh_range: ParameterRange::new(6.5, 8.0),
            nitrate_range: ParameterRange::new(5.0, 15.0),
            nitrite_range: ParameterRange::new(0.0, 10.0),
            ammonia_range: ParameterRange::new(0.0, 10.0),
        }
    }
}

#[derive(Debug)]
pub enum FishTier {
    Nano,
    Community,
    Tropical,
    Predator,
    RiverMonster,
}

#[derive(Debug)]
pub enum FishStatus {
    Thriving,
    Healthy,
    Neatural,
    Sick,
    Critical,
}

#[derive(Debug)]
pub struct Fish {
    pub species: String, //Mostly just a display for the users
    pub age: u32,
    pub max_age: u32, //Have this be some random range each fish
    pub min_group: u8,
    pub tier: FishTier,
    pub fish_traits: Vec<traits::Trait>,

    pub tolerances: Tolerances,

    pub moddifiers: Vec<Moddifier>,
    pub hunger: f64,
    pub status: FishStatus, //calculated based on w wellness score from the tank parameters
    pub wellness: f64, //the int that will drive the status calculation
}

impl Fish {
    pub fn new() -> Fish {
        Fish {
            species: String::from("Guppy"),
            age: 0,
            max_age: 100, //Have this be some random range each fish
            min_group: 3,
            tier: FishTier::Nano,
            fish_traits: vec![
                traits::Trait::new(traits::TraitNames::AmmoniaBoost, 1.05, 60.0)
            ],

            tolerances: Tolerances::new(),

            moddifiers: vec![
                Moddifier::new(tank::WaterParameter::ammonia, 0.5),
            ],
            hunger: 50.0,
            status: FishStatus::Neatural, //calculated based on w wellness score from the tank parameters
            wellness: 60.0, //the int that will drive the status calculation
        }
    }

    /*
        This calculation needs to check if each paramter of the water is within this fish's range
        if its not then we need to take the differenec and multiply by the WELLNESS_PENALTY_SEVERITY
        the calculation starts at 60.0 points to make thriving a special and healthy hard to get to.
        This should be changeable as needed to otweak the formula
    */
    pub fn calculate_wellness(&self, water: &tank::WaterParameters) -> f64 {
        let mut score = 60.0; //Starts the fish at neutral

        score -= self.calculate_parameter_penalty(water.temprature, &self.tolerances.temprature_range);
        score -= self.calculate_parameter_penalty(water.ph, &self.tolerances.ph_range);
        score -= self.calculate_parameter_penalty(water.gh, &self.tolerances.gh_range);
        score -= self.calculate_parameter_penalty(water.nitrate, &self.tolerances.nitrate_range);
        score -= self.calculate_parameter_penalty(water.nitrite, &self.tolerances.nitrite_range);
        score -= self.calculate_parameter_penalty(water.ammonia, &self.tolerances.ammonia_range);      

        score
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
}