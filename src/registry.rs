use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeRange {
    pub min: u32,
    pub max: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeciesTrait {
    pub trait_name: String,
    pub multiplier: f64,
    pub weight: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modifier {
    pub parameter: String,
    pub modifier: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeciesTolerance {
    pub temperature_range: SpeciesParameterRange,
    pub ph_range: SpeciesParameterRange,
    pub gh_range: SpeciesParameterRange,
    pub nitrate_range: SpeciesParameterRange,
    pub nitrite_range: SpeciesParameterRange,
    pub ammonia_range: SpeciesParameterRange,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeciesParameterRange {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FishSpecies {
    pub species: String,
    pub max_age_range: AgeRange,
    pub min_group: u8,
    pub tier: String,
    pub traits: Vec<SpeciesTrait>,
    pub modifiers: Vec<Modifier>,
    pub base_prestige: f64,
    pub tolerances: SpeciesTolerance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FishRegistry {
    pub fish: Vec<FishSpecies>
}

impl FishRegistry {
    pub fn load() -> FishRegistry {
        let bytes = std::fs::read("data/fish_registry.bin").expect("[ERR]Reading data/fish_registry.bin");
        bincode::deserialize(&bytes).expect("[ERR] Could not deserialize bytes")
    }
}