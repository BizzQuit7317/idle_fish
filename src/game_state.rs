use crate::tank;
use crate::traits;
use crate::player;
use crate::registry;
use crate::economy;
use crate::ui_helper;
use crate::offline_report;
use crate::debug;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub tank: tank::Tank,
    pub player: player::Player,
    pub economy: economy::Economy,
    pub notification: ui_helper::Notification,
    pub offline_report: offline_report::OfflineReport,
    pub fish_registry: registry::FishRegistry,
    pub debugger: debug::Debugger,
}

impl GameState {
    pub fn new() -> GameState {
        let mut state = GameState {
            tank: tank::Tank::new(),
            player: player::Player::new(),
            economy: economy::Economy::new(),
            notification: ui_helper::Notification::new(),
            offline_report: offline_report::OfflineReport::new(),
            fish_registry: registry::FishRegistry::load(),
            debugger: debug::Debugger::new(),
        };

        //Need to push the fish registry to the tank so it can add fish
        //if let Some(species) = state.fish_registry.fish.iter().find(|s| s.species == "Swordtail") {
        //    state.tank.fish.push(fish::Fish::new(species));
        //}

        state
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

        //Third pass, calculate each fish's wellness
        for fish in &mut self.tank.fish {
            //println!("[DBG] Checking wellness calculation");
            //Do all checks on each fish indivdually
            fish.calculate_wellness(&self.tank.water_parameters);
            fish.status_check();
            fish.calculate_hunger();
            fish.increase_age();

            self.player.current_prestige += fish.get_prestige();
            self.player.all_time_prestige += fish.get_prestige();

            fish.alive_check(); //check alive state last so they get to live out their last year and player gets points for it
            //println!("[DBG] Fish huger {}\n~~~~~~~~~~~~~~~~~~~~~~~~~", fish.hunger);
            //Remove the traits of dead fish on the tank
            if !fish.alive {
                for fish_trait in &fish.fish_traits {
                    match fish_trait.trait_name {
                        traits::TraitNames::TempratureBoost => self.tank.water_parameters.temprature /= fish_trait.multiplier,
                        traits::TraitNames::AmmoniaBoost => self.tank.water_parameters.ammonia /= fish_trait.multiplier,
                        traits::TraitNames::PHBoost => self.tank.water_parameters.ph /= fish_trait.multiplier,
                        traits::TraitNames::GHBoost => self.tank.water_parameters.gh /= fish_trait.multiplier,
                        traits::TraitNames::NitrateBoost => self.tank.water_parameters.nitrate /= fish_trait.multiplier,
                        traits::TraitNames::NitriteBoost => self.tank.water_parameters.nitrite /= fish_trait.multiplier,
                    }
                }
            }
        }
        //println!("###########################################");
        //Run the tanks Nitrogen Cycle and PH drift
        self.tank.nitrogen_cycle();
        self.tank.ph_drift();
        self.tank.gh_depletion(); //Fih eating away at the minerals in the water

        //Algea only gains light when its on
        if self.tank.lighting.on {
            self.tank.algea_colony.grow(self.tank.lighting);
        }

        println!("[DBG]current algea light levels: {}", self.tank.algea_colony.light_levels);

        self.tank.parameter_clamp(); //Finally clamp all parameters to nt go beyond limits

        //take a snapshoot of list len before removeing fish for tracking fish deaths, MUST ADD FISH BEFORE THIS CHECK
        let pre_death_fish_len = self.tank.fish.len() as u32;

        //Finally we need to remove any fish that have died in the tank
        if self.tank.check_fish() {
            self.tank.update_ideal_parameters();
        }

        //check the players total fish here to account for all the removed fish
        self.player.current_fish_owned = self.tank.fish.len() as u32;
        self.player.total_fish_died += pre_death_fish_len - self.player.current_fish_owned;

        //println!("[DBG] Player pretige points {} Tank parameters {:?} the player has {} fish currently and {} dead fish overall\n###################", &self.player.current_prestige, &self.tank.water_parameters, &self.player.current_fish_owned, &self.player.total_fish_died); //only here for debugging to see each tank
    }
}