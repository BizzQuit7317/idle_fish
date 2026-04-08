use crate::fish;

#[derive(Debug)]
pub enum WaterParameter {
    temprature,
    ph,
    gh,
    nitrate,
    nitrite,
    ammonia,
}

#[derive(Debug)]
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
        WaterParameters {
            temprature: 25.0,
            ph: 7.0,
            gh: 10.0,
            nitrate: 0.0,
            nitrite: 0.0,
            ammonia: 10.0,
        }
    }

    pub fn apply_changes(&mut self, parameter: &WaterParameter, value: f64) {
        match parameter {
            WaterParameter::temprature => self.temprature -= value,
            WaterParameter::ph => self.ph -= value,
            WaterParameter::gh => self.gh -= value,
            WaterParameter::nitrate => self.nitrate -= value,
            WaterParameter::nitrite => self.nitrite -= value,
            WaterParameter::ammonia => self.ammonia -= value,
        }
    }
}

#[derive(Debug)]
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

            //define fish stats
            max_fish: 10,
            fish: vec![
                fish::Fish::new(),
                //fish::Fish::new(),
                //fish::Fish::new(),
            ],
        }
    }
}
