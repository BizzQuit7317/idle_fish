<div align="center">
```
╔══════════════════════════════════════════╗
║   🐠  i d l e _ f i s h  🐠            ║
║   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~      ║
║   the aquarium idle game nobody asked   ║
║   for but everyone deserves             ║
╚══════════════════════════════════════════╝
```
 
*prestige points. nitrogen cycles. fish vibes.*
 
[![Status](https://img.shields.io/badge/status-actively_neglecting_my_virtual_fish-blue?style=flat-square)](.)
[![Genre](https://img.shields.io/badge/genre-idle%20%2F%20incremental-teal?style=flat-square)](.)
[![Inspiration](https://img.shields.io/badge/inspired_by-NGU%20%7C%20USI-purple?style=flat-square)](.)
 
</div>
---
 
## 🌊 What is this?
 
An aquarium idle game. You keep fish. Fish generate prestige. Prestige unlocks more fish. The tank gets out of hand. You reset. You do it again. You tell yourself *this time you'll optimise better.*
 
Much like NGU Idle or Universal Sploder Idle, the fun is entirely in the eye of the beholder. If you enjoy watching numbers go up while managing nitrogen cycles and debating whether a second filter is worth the component slot — **this game is for you.**
 
---
 
## 🧪 The Chemistry (it's actually real)
 
idle_fish simulates a real-ish nitrogen cycle because we're committed to making you learn biochemistry by accident.
 
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
Managing this is literally the core loop. Your fish's wellness depends on how well you keep the water parameters in check.
 
---
 
## 🐟 Fish
 
Each fish works like a v-device from USI — they have two things going on:
 
| Property | What it does |
|----------|-------------|
| **Trait** | A one-time prestige multiplier on purchase |
| **Modifier** | A passive modifier to your tank parameters |
 
Fish also have:
- **Tolerances** — temperature, pH, GH, nitrate, nitrite, ammonia ranges they can survive in
- **Hunger** — scales faster the lower it gets, also affected by wellness
- **Age** — each species ages differently
- **Wellness** — calculated from water params, hunger, and age
---
 
## 🗂️ Tabs
 
| Tab | Status | Description |
|-----|--------|-------------|
| 🪣 **Tank** | Always available | Core stats, feeding, fish assignment |
| 🐠 **Fish** | Always available | Manage your fish (think: NGU Diggers page) |
| ⚙️ **Components** | Always available | Filters, heaters, and other hardware |
| 🏪 **Store** | Always available | Buy fish, food, components |
| 🪸 **Decorations** | Unlock early | Passive bonuses (like v-devices from USI) |
| 🧴 **Chemicals** | Mid-game | Spike or adjust water params; Quick Start on rebirth |
| 🏆 **Prestige** | Unlockable | Rebirth — reset tank to RO water, keep some bonuses |
| 🔧 **Sump** | Late game | Extra component slots + special fish for big multipliers |
 
---
 
## 🔁 Game Loop
 
```
Start: Empty tank of RO water at room temperature
         │
         ▼
    Buy components
    (filter, heater, etc.)
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
 
**Fish data** is stored as a binary file to stop people editing their save like little cheats. A converter script takes a friendly `.json` and spits out the `.bin` the game actually uses.
 
### Fish JSON structure (for the converter)
 
```json
{
  "species": "neon_tetra",
  "max_age_range": { "min": 5000, "max": 8000 },
  "min_group": 6,
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
 
### 🔧 In Progress
- [ ] Hunger formula (scales faster at low values, affected by wellness)
- [ ] Per-species age rates (`age_rate` marker)
- [ ] Wellness calculation factoring in hunger + age (not just water params)
- [ ] Different foods with different hunger values
- [ ] Food drop animation
- [ ] Offline time limit + proper offline report popup
- [ ] More fish species
- [ ] Fish swap in test menu
- [ ] Nicer notification pop-ups
### 🐛 Known Bugs
- [ ] Tab click causes all later-loaded elements to flash briefly
- [ ] Store has a long initial load
- [ ] `file_control` panics and crashes if save file is missing
- [ ] Water change cooldown notification shows total cooldown, not time remaining
---
 
## 🙏 Inspired By
 
- **[NGU Idle](https://store.steampowered.com/app/1073970/NGU_IDLE/)** — for the diggers-style management UI and general idle insanity
- **[Universal Sploder Idle](https://www.kongregate.com/games/StickMan6040/universal-paperclips)** — for the v-device / trait + modifier system
---
 
<div align="center">
*keep your nitrates low and your fish happy* 🐡
 
</div>
