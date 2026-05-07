# idle_fish
Fish keeping idle game, simmillar to NGU or USI
Build your aquarium with differnt fish to get the most prestigious. Your goal is to have the most efficent aquarium or some shit. much like USI or NGU the fun 
is in the eye of the beholder

## Idea
Each fish is sort of like a v-device from USI, it has the trait which does a 1 time multiplier, and the mod which runs a constnat mod on tank params, the tank should have a series of 
penalties for like fish or over feeding or whatever that contribute to the nitate cycles and such

## Tabs 
- Tank (all the basic stats, can feed fish here, choose what fish are in the tank or in stock and more)
- Fish (A page to specifically manage the fish in your tank, maybe somthing like the diggers page on NGU)
- Components (simmillar to fish but for components like filters and heaters)
- Store (where you can buy all your fish, food and others)
All tabs from this point on should be unlocked at various stages
- Decorations (v-devices from USI, unlock early)
- Chemicals (not sure of an equivelent but allows the player to add chemicals to the water to spike or change certain things, maybe at somepoint gives the player an option to use like quick start on rebirth)
- Prestige page (not sure if this should be unlocked or not, but should just be a rebirth, some stats are kept but the tank is set to basic RO water again, typicall rebirth stuff)
- Sump (potentially mid or late game expansion that gives the player a sump where they can add additional components and special fish for massive bonuses)

## General Game loop
At the start the player just has a empty tank of RO water, it sitting at room temprature, the player must use their money to buy some components and soome fish
The fish will generate prestige points which can be used to unlock and buy more components and fish.
If you feel like your tank is getting slow or starting to build to much debre you can restart keeping prestige points and getting slow upgrades.

## To Do
- Need to create and implement formuals for the following fish stats
- Hunger (should scale so it goes quicker the lower the number, should also scale with wellness)
- Age fish should all age slightly differntly (might need another fish marker for age_rate)
- Wellness calculation should also take into account the other factos like hunger and age instead of just water parameters
- Add food differnt foods
- Add differnt hunger increae for each foodd
- Add animation for food dropping
- Prettify the notification pop up
- Add offline time limit for player
- Add more detail to offline report#
- Make ffline report a poppup intead of cmd dbg
- Add more fish
- Add some way to swap between fih in the test menu

## Bugs
- When you click a tab all elements loaded after it flash for some reason

## General
The fish data is stred as a binary file to stop player playing around with it, use a converter script to convert a json file that is easy to work with into a bin file you can use


## Fish json struct for converting
```
{
      "species": "", 
      "max_age_range": { "min": , "max":  }, //IN game ticks of life, around 1 second per tick
      "min_group": ,
      "tier": "",
      "traits": [
        { "trait_name": "", "multiplier": , "weight":  }
      ],
      "modifiers": [
        { "parameter": "", "modifier":  }
      ],
      "base_prestige": ,
      "base_cost": ,
      "tolerances": {
        "temperature_range": { "min": , "max":  },
        "ph_range": { "min": , "max":  },
        "gh_range": { "min": , "max":  },
        "nitrate_range": { "min": , "max":  },
        "nitrite_range": { "min": , "max":  },
        "ammonia_range": { "min": , "max":  }
}
```
