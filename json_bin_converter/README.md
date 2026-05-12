# fish_registry converter
 
JSON → binary converter for the idle_fish `fish_registry`. Part of the [idle_fish](../README.md) project.
 
![Language](https://img.shields.io/badge/language-Rust-orange?style=flat-square)
![Format](https://img.shields.io/badge/input-JSON-lightgrey?style=flat-square)
![Output](https://img.shields.io/badge/output-bincode-teal?style=flat-square)
 
---
 
## What it does
 
Takes `fish_registry.json`, deserialises it into the `FishRegistry` struct, and writes it out as a bincode `.bin` file. The game reads the `.bin` directly — the JSON is just the human-editable source of truth.
 
---
 
## Usage
 
Drop your `fish_registry.json` into `src/`, then run:
 
```bash
cargo run
```
 
Outputs `src/fish_registry.bin`. Done.
 
---
 
## Adding new fields
 
If you add anything to the JSON (new field, new section, whatever) you need to mirror it in **two places**:
 
1. The relevant struct(s) in `main.rs` — `FishSpecies`, `SpeciesTolerance`, `SpeciesTrait`, etc.
2. The corresponding structs in the **main game** (`registry.rs`) — they must stay in sync or bincode will fail to deserialise
---
 
## Structs
 
```
FishRegistry
└── Vec<FishSpecies>
    ├── species, tier, min_group, base_prestige
    ├── max_age_range       → AgeRange { min, max }
    ├── traits              → Vec<SpeciesTrait { trait_name, multiplier, weight }>
    ├── modifiers           → Vec<Modifier { parameter, modifier }>
    └── tolerances          → SpeciesTolerance
            ├── temperature_range
            ├── ph_range
            ├── gh_range
            ├── nitrate_range
            ├── nitrite_range
            └── ammonia_range   → SpeciesParameterRange { min, max }
```
