use std::fs;
use serde::{Deserialize, Serialize};

// paste all your structs from registry.rs here
// AgeRange, SpeciesTolerance, SpeciesParameterRange, 
// SpeciesTrait, SpeciesModifier, FishSpecies, FishRegistry

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

fn main() {
    let json = fs::read_to_string("src/fish_registry.json")
        .expect("Could not read fish_registry.json");
    
    let registry: FishRegistry = serde_json::from_str(&json)
        .expect("Could not parse fish_registry.json");
    
    let binary = bincode::serialize(&registry)
        .expect("Could not serialize to binary");
    
    fs::write("src/fish_registry.bin", binary)
        .expect("Could not write fish_registry.bin");
    
    println!("Successfully wrote src/fish_registry.bin with {} species", registry.fish.len());
}