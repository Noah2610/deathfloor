# Design notes
## General
- Room traversal
  After a short transition teleport player to new room. new room is seperate tmx / json, so different rooms can have different room sizes / camera settings / zoom level?

### Player Movement
- Jump 
- Walljump
- Slide:
  Change hitbox and gain static velocity for duration of slide. spammable?

### Environmental Animations
- Animated Tiles (basic loop, ping pong, animation triggered by specific event)
- Animated Backgrounds
- "Particle Effects":
  Small animated textures with white space are being spawned at location where trigger happens, despawn after going through animation cycle once. Can be for example action lines when jumping or static on the floor when walking over it in some electricity themed level.
  

## Combat

### Health
Number that can be be added to or subtracted from on certain events. Visually corresponding texture in UI. Set max health. 

### I-frames
When the player takes any sort of damage, an animation plays (or maybe shader?) during which the player is invincible to incoming damage. 

### Player death
A short animation plays and the player is prompted with a screen asking them to either retry the level or to return to the menu.
If player still has lives left, they respawn at the most recent checkpoint and lose 1 life.

## Enemies 
- Enemy Death:
  Entity gets destroyed, a short animation and maybe sound is played, optionally they drop something (for example a health pack)
- Spawning enemies
  
### Components

- Enemies consist of various components that can be combined to craft simple behavior. 

- Damage on collision:  
  Deals damage to the player on collision.
- Basic roaming:  
  Move left and right (initial direction can be set in tiled) until hitting either a ledge or a wall, then pivot. 
- 360 roaming:
  Move in one direction (can be set in tiled), when encountering a ledge or wall move upwards / downwards along it and ignore gravity.
- Basic air roaming:  
  Move along a set path. When reaching end of path, pivot. (Path is set manually in tiled)
- Basic shooting:  
  Spawns projectiles in walking direction in a set interval.
- Basic LOS react:  
  When player enters their LOS (LOS as in 1 rectangular hitbox that is being projected in front of them in walking direction) do something else / switch state. For example Basic Shooting. 
- Basic charging:  
- Delay:  
  A manually set time that makes the enemy loops what its currently doing before continuing on with next behavior. Can be set "between" 2 behavioral states / at the transition between them. For example: Basic react shooting enemy spots player. Instead of instantly entering shooting state, the delay is run first. Then enter next state, in this case shooting.
- Explode:  
  Self destroy and deal AOE damage.
- On death:  
  Define if and if yes, which component is activated when enemy dies, for example explode.
- On spawn:  
  Define component(s) / state(s) that are active when enemy spawns / the starting states.
- Splitting:  
  Enemy is split into multiple, individual parts and only share some of their components. -> enemy has multiple hitboxes and respective hp pools, but same movement component - so they "move as one".
- The other kind of splitting:  
  "Split" into multiple smaller enemies (play animation, spawn new enemies, destroy current enemy), for example on death. 
- On impact:  
  When hitting a wall, do something.
- Basic chasing:
  When player enters LOS enemy moves towards player (flying), when player leaves LOS they freeze (until Player reenters LOS)
- Jumpable: 
  Player can jump off of them
- Drop: Drop something, for example on death drop health pack. 
- Animation: Play an animation
- Random Jump: "Jump" in set interval in semi random directions (either randomly select from a pool of manually set x and y values or generate new ones)
- Stick: When hitting a solid, freeze in place and ignore gravity. Can for example be combined with random jump. 
- Drop: Stop exectuing movement component when player enters LOS that is projected from enemy in "gravity direction" (shouldnt be hardcoded downwards but actually take current gravity direction in case gravity walls will be a thing) and drop downwards. Can for example be combined with 360 roaming, on impact and explode for dropping bomb traps. 
- Spawner: Spawn seperate enemies at their location in set interval. how long the interval is and which enemies are being spawned is manually set. 

## Environmental Mechanics
- Jumppad:  
  Applies set amount of velocity to player. 
- One way platforms  
- Ladders:
  If player inputs up while in collision with ladder an animation is triggered and gravity is killed. can now move upwards and downwards at constant speed or left or right / jump to leave ladder state. 
- Spikes:  
- Turrets:  
  Spawn projectiles of set kind (texture, hitbox size, damage, etc...) in set interval. Can also project rectangular, static hitbox in front of them in set interval (for example for air leaking out of a cracked pipe or a flamethrower).
- Disappearing platforms:  
  Disappear on set event, for example on contact with player (after short delay). 
- Basic moving platforms:  
  Move along manually set path until reaching end of it, then pivot. 
- Advanced moving platforms:
  On contact with player start moving along manually set path until reaching end of it, then freeze. Repeat.
- Pressure plates:  
  Do something while Player is standing on them, for example move platform. 
- Knockbackers:  
  Apply set velocity to player upon entering rectangular hitbox. Can for example be used for pipe leak air. 
- Gravity Walls:
  Placed as tiles in editor. When players jump on walls, their sprite + hitbox rotates. Also: room gravity changes, controls change, texture of the tiles gets swapped (tiles of current floor get a glow or something) ("animated"?). Controls get remapped to make movement more intuitive.
- Low grav
- States:
  Some environmental mechanics can have multiple states. For example: Platform has one state in which it is solid and one in which it isn't and has a different texture. 
- Cycling:
  Can cycle through states either in set interval or tied to other event happening.

## Weapons
- Weapon switch: Have different weapon types between which you can switch ingame. 

### Weapon types
- Pistol: 
  single shot, spammable up to three shots, then cooldown
- Strong pistol: 
  single shot, not spammable, high damage
- Shotgun: 
  single shot, not spammable, 3 low damage projectiles (the outer two being slightly angled)
- Strong shotgun: 
  same as shotgun, but: applies knockback to player, so player can use it as means to move around
- Semi auto:
  Shoots in salves of 3, can be charged to bundle the 3 projectiles into a single high damage one. 
- Grenade launcher:
  shoots projectile that explodes when it hits wall. is gravity affected. can be charged up to manipulate how fast the projectile gets shot. -> explosive jumping. explosives deal aoe damage in small radius or rectangle and knockback player in larger radius or rectangle around the explosion. can be used either as environmental hazard or movement mechanic. 
- Basic Melee:
  deals damage within rectangular hitbox that is projected in front of player in walking direction. short cooldown inbetween swings. can deflect enemy projectiles.

## SFX
- Possible events / actions that have SFX
- Shooting
- Jumping
- Walking
- Jumppads
- Moving Platforms (for example when they reach the end of their path)
- Room to Room Transition
- Various UI / Menu Elements
- Picking up Health Packs or Powerups
- Death sound / jingle
- Explosions
- Charging weapons