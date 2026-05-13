```
╔══════════════════════════════════════════╗
║   🐠  i d l e _ f i s h  🐠            ║
║   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~      ║
║   the aquarium idle game nobody asked   ║
║   for but everyone deserves             ║
╚══════════════════════════════════════════╝
```             

 
*prestige points. nitrogen cycles. fish vibes.*
 
[![Status](https://img.shields.io/badge/status-actively_thinking_about_adding_shrimps-blue?style=flat-square)](.)
[![Genre](https://img.shields.io/badge/genre-idle%20%2F%20incremental-teal?style=flat-square)](.)
[![Inspiration](https://img.shields.io/badge/inspired_by-NGU%20%7C%20USI-purple?style=flat-square)](.)
 
</div>
---
 
## 🌊 What is this?
 
An aquarium idle game. You keep fish. Fish generate prestige. Prestige unlocks more fish.*
 
Much like NGU Idle or Unnamed Space Idle, you need to try and balance an array of stats and parameters to increasse your numbers!
 
---
 
## 🧪 The Chemistry
 
idle_fish simulates a real-ish nitrogen cycle.
 
```
AMMONIA ──(passive)──► NITRITE ──(slow)──► NITRATE
   │                                           │
   │ raises pH                    lowers pH    │
   │                                           │
   └─── pH flux → equilibrium ◄───────────────┘
                                      ▲
                              needs water change
                              to clear nitrate
```
 
**The cycle in plain English:**
- Your fish poop → produces **Ammonia** → bad, raises pH
- Ammonia converts to **Nitrite** → also bad, lowers pH
- Nitrite converts to **Nitrate** → less bad, but builds up
- You do a **water change** to reset nitrate, or your fish suffer
Managing this is the core loop. Your fish's wellness depends on how well you keep the water parameters in check.
 
---
 
## 🐟 Fish
 
Each fish works like a equip item — they have two things going on:
 
| Property | What it does |
|----------|-------------|
| **Trait** | A one-time multiplier on addition to tank |
| **Modifier** | A passive modifier to your tank parameters per tick |
 
Fish also have:
- **Tolerances** — temperature, pH, GH, nitrate, nitrite, ammonia ranges they can survive in
- **Hunger** — scales faster the lower it gets, also effects wellness
- **Age** — each species ages differently
- **Wellness** — a calculated average between the hunger and parmeter ranges
---
 
## 🗂️ Tabs
 
| Tab | Status | Description | Built status |
|-----|--------|-------------|--------------|
| 🪣 **FishStatus** | Always available | Stats of all active fish in tank | Built |
| 🐠 **Store** | Always available | Area to buy fish and core upgrades) | Built |
| ⚙️ **Components** | Unlock early | Filters, heaters, and other hardware | Not Built |
| 🪸 **Decorations** | Unlock early | Passive bonuses (like v-devices from USI) | Not Built |
| 🧴 **Chemicals** | Mid-game | Spike or adjust water params; Quick Start on rebirth | Not Built |
| 🏆 **Prestige** | Unlockable | Rebirth — reset tank to RO water, keep some bonuses | Not Built |
| 🔧 **Sump** | Late game |  its own mini eco system with it own components, chemicals carry over, and sump specific fish | Not Built |

 - More tabs to come in future
---

## ⚙️ Settins Tabs
 
| Tab | Status | Description | Built status |
|-----|--------|-------------|--------------|
| 🪣  **Game** | Always available | Options for the stats, partially a debug menu for now | Built |
| 🏆  **Player Stats** | Always available | A plain text view of all the player stats for future debugging | Built |
| 🔧 **Options** | Always available | A proper options or moore standard settings page | Not Built |

 - More tabs to come in future
---
 
## 🔁 Game Loop
 
```
Start: Empty tank of RO water at room temperature
         │
         ▼
    Buy fish → fish generate prestige points
         │
         ▼
    Manage water params to keep fish happy
    (water changes, chemicals, feeding)
         │
         ▼
    Unlock more fish / components / tabs
         │
         ▼
    Tank getting gross? Numbers slowing down?
         │
         ▼
    PRESTIGE → reset tank, keep prestige upgrades
         │
         └──────────────────────────────────► repeat
```
 
---
 
## 🛠️ Tech Notes
 
**Fish data** is stored as a binary file to stop people editing their save like little cheecky bastards. A converter script takes a friendly `.json` and spits out the `.bin` the game actually uses (see the json_bin_converter dir).
 
### Fish JSON structure (for the converter)
 
```json
{
  "species": "neon_tetra",
  "max_age_range": { "min": 5000, "max": 8000 },
  "min_group": 6,
  "min_tank_size": 3,
  "tier": "common",
  "traits": [
    { "trait_name": "schooling", "multiplier": 1.4, "weight": 0.6 }
  ],
  "modifiers": [
    { "parameter": "ammonia", "modifier": 0.02 }
  ],
  "base_prestige": 10,
  "base_cost": 50,
  "tolerances": {
    "temperature_range": { "min": 20, "max": 26 },
    "ph_range": { "min": 6.0, "max": 7.5 },
    "gh_range": { "min": 2, "max": 10 },
    "nitrate_range": { "min": 0, "max": 20 },
    "nitrite_range": { "min": 0, "max": 0.5 },
    "ammonia_range": { "min": 0, "max": 0.25 }
  }
}
```
 
> ⚠️ Age is measured in game ticks (~1 second per tick)
 
---
 
## 📋 Roadmap
 
### 🔧 In Progress Main
- [ ] Build Debugger/Cheats page in settings
- [ ] Balance numbers
- [ ] Add light cycle into game
- [ ] Add Algea system into game
### 🔧 In Progress Side/Later
- [ ] Per-species age rates (`age_rate` marker)
- [ ] Offline time limit + proper offline report popup
- [ ] More fish species
- [ ] More robust prestige calculation per fish
### 🔧 In Progress QoL
- [ ] Nicer notification pop-ups
- [ ] Food drop animation
- [ ] Move all flat file struct in rust into dirss for more visual clarity
### 🐛 Known Bugs
- [ ] Tab click causes all later-loaded elements to flash briefly
- [ ] Store has a long initial load
- [ ] `file_control` panics and crashes if save file is missing
- [ ] Water change cooldown notification shows total cooldown, not time remaining
- [ ] Settings page does not continue counting while the player is checking it
- [ ] In settings the scroll aaction clips into the tabs
- [ ] Potential bug if the fish tier is misspelt or changed from outside the enum it defaults to nano, could hide typos or cause headache in future updates
---
 
## 🙏 Inspired By
 
- **[NGU Idle](https://store.steampowered.com/app/1073970/NGU_IDLE/)** — for the diggers-style management UI and general idle insanity
- **[Unnamed Space Idle](https://store.steampowered.com/app/2471100/Unnamed_Space_Idle/)** — for the v-device / trait + modifier system
---
 
<div align="center">
*keep your nitrates low and your fish happy* 🐡
 
</div>
