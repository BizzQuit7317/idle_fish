use crate::tank;
use crate::traits;

#[derive(Debug)]
pub struct GameState {
    pub tank: tank::Tank,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            tank: tank::Tank::new(),
        }
    }

    /*
        The tick function needs to loop over the fish and apply each change
        the order of event is imporant so be mindful when makeing changes here
        we want to apply changes to WaterParamters before we move to the next 
        set of changes
    */
    pub fn tick(&mut self) {
        //First pass, apply all the fissh moddifier to the tanks water
        for fish in &self.tank.fish {
            for modd in &fish.moddifiers {
                self.tank.water_parameters.apply_changes(&modd.parameter, modd.moddifier);
            }
        }

        //Second pass, needs to new apply the traits to the water
        for fish in &self.tank.fish {
            for fish_trait in &fish.fish_traits {
                match fish_trait.trait_name {
                    traits::TraitNames::TempratureBoost => self.tank.water_parameters.temprature *= fish_trait.multiplier,
                    traits::TraitNames::AmmoniaBoost => self.tank.water_parameters.ammonia *= fish_trait.multiplier,
                    traits::TraitNames::PHBoost => self.tank.water_parameters.ph *= fish_trait.multiplier,
                    traits::TraitNames::GHBoost => self.tank.water_parameters.gh *= fish_trait.multiplier,
                    traits::TraitNames::NitrateBoost => self.tank.water_parameters.nitrate *= fish_trait.multiplier,
                    traits::TraitNames::NitriteBoost => self.tank.water_parameters.nitrite *= fish_trait.multiplier,
                }
            }
        }

        //Third pass, calculate each fish's wellness
        for fish in &mut self.tank.fish {
            fish.wellness = fish.calculate_wellness(&self.tank.water_parameters);
        }

        println!("Tank parameters {:?}\n###################", &self.tank.water_parameters); //only here for debugging to see each tank
    }
}