## A comphrehensive collection of desired features


### Environmental mechanics:
- Spikes
- Jumppads
- Ladders:
  When in collision and player inputs "up", player gets teleported on x to center of ladder. -> enter ladder "state". while on ladder, player ignores gravity and can move upwards and downwards with constant velocity. Player can leave ladder by giving horizontal input and / or jumping.
- Moving platforms:
  Solids with rectangular hitbox that move along set path and can push and carry entities like the player or enemies. Being able to define the path through tiled's path tool (or having a comparable level of control over how the path is layed out) would be great. 
- Turrets:
  Stationary or moving (path would be set the same way as for moving platforms) "enemies" that shoot set type of projectile in set interval. Have hp. 
- "Damage fields":
  Rectangular collision boxes within which a sprite is displayed and damage is applied to the player. Interval can be set for fields to switch between "on and off". (Use case / visual execution would for example be a flamethrower or a gas leak or an electric "barrier" / laser)
- Knockbackers:  
  Apply set velocity to player upon entering rectangular hitbox. Can for example be used for pipe leaking air.
- Pressure plates
  Check for initial or ongoing(!) collision with player (and / or enemies?) and, if checked positively, execude additional logic (depending on specific use either once or while(!) / as long as collision check returns true). Possible use cases: Platforms that only move while player is standing on them or different kinds of booby traps (for example spikes that activate / "come out of the ground" (would be a simple animation) (would probably make sense to use damage fields instead of actual spikes)) 
- Disappearing platforms:  
  Disappear on set event, for example on contact with player (after short delay). 
- Low grav
- States:
  Some environmental mechanics can have multiple states. For example: Platform has one state in which it is solid and one in which it isn't and has a different texture. 
- Cycling:
  Can cycle through states either in set interval or tied to other event happening.
- Water:
  "Field" that has texture and different gravity + friction settings.
- Ice
- Explosive boxes:
  Explode when afflicted with any kind of damage (from any source)  


### Player mechanics (Movement):

- Jump
- Walljump
- Slide:
  Temporarialy change player hitbox, sprite and velocity. Can be cancled with a jump. Movement direction and velocity is locked for the duration of the slide.
- Air dash:
  Can only be performed while in air. Standard dash, set constant directional velocity and ignore gravity for duration of dash. 1, 2, 4, 8 directional?


### Enemy mechanics:

- 360 roaming:
  Move in one direction (can be set in tiled), when encountering a ledge or wall move upwards / downwards along it and ignore gravity.
- Basic air roaming:  
  Move along a set path. When reaching end of path, pivot. (Path is set manually in tiled)
- Basic LOS react:  
  When player enters their LOS (LOS as in 1 rectangular hitbox that is being projected in front of them in walking direction) do something / change state. For example Basic Shooting. 
- Basic charging:  
- Splitting:  
  Enemy is split into multiple, individual parts and only share some of their components. -> enemy has multiple hitboxes and respective hp pools, but same movement component - so they "move as one".
- The other kind of splitting:  
  "Split" into multiple smaller enemies (play animation, spawn new enemies, destroy current enemy), for example on death. 
- Stick: When hitting a solid, freeze in place and ignore gravity. Can for example be combined with random jump. 
- Drop: Stop exectuing movement component when player enters LOS that is projected from enemy in "gravity direction" (shouldnt be hardcoded downwards but actually take current gravity direction in case gravity walls will be a thing) and drop downwards. Can for example be combined with 360 roaming, on impact and explode for dropping bomb traps. 
- Chaser: When player enters their LOS, chase "charge" at player (like ghosts in stabman) with slight delay in movement. Can for example be combined with explode for kamikaze enemies. 
- Spawner: Spawn seperate enemies at their location in set interval. how long the interval is and which enemies are being spawned is manually set. 
- Jump: "Jump" in set interval in semi random directions (either randomly select from a pool of manually set x and y values or generate new ones)  
- For other enemy mechanics, see enemy section in DESIGN.md


### Weapon mechanics (Some for player, some for enemies, most would work for both):

- Basic Projectile
  Variations: Single shot, burst of 3, continuous fire, shotgun (single shot, 3 projectiles - 2 of them angled)
- Chargable
  Shoot input can be held to charge up projectile (changing various properties of the projectile, for example sprite, damage, size, speed, etc...)
- Explosives
  Deal AOE damage on impact (with entities or solids) and spawn sprite (explosion animation that is played once)
- Knockback
- Gravity affected projectiles:
  Can for example be given both x and y velocity when spawned / fired in order to have an arched path
- Freeze projectiles:
  Freeze entities on impact ("stun" them, set ice block sprite, and give "ice physics") (could, like all other projectile types, be combined with "chargable" to impact freeze duration)