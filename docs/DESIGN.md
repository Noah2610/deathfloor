# Design notes
## General

## Combat

### Health
Number that can be be added to or subtracted from on certain events. Visually corresponding texture in UI. 

### I-frames
When the player takes any sort of damage.

### Player death
A short animation plays and the player is prompted with a screen asking them to either retry the level or to return to the menu.
If player still has lives left, they respawn at the last checkpoint and lose 1 life.

### Enemy death
Their hitbox is removed, a short animation plays. Optionally an item is dropped (for example a health pack).

## Enemies
### Components:
- Damage on collision  
  Deals damage to the player on collision.
- Basic roaming  
  Move left and right until hitting either a ledge or a wall, then pivot. 
- Basic air roaming  
  Move along a set path. When reaching end of path, pivot. (Path is set manually in tiled)
- Basic shooting  
  Spawns projectiles in walking direction in a set interval.
- Basic LOS react  
  When player enters their LOS (LOS as in 1 rectangular hitbox that is being projected in front of them in walking direction) do something else / switch state. For example Basic Shooting. 
- Basic charging  
- Delay  
  A manually set time that makes the enemy loops what its currently doing before continuing on with next behavior. Can be set "between" 2 behavioral states / at the transition between them. For example: Basic react shooting enemy spots player. Instead of instantly entering shooting state, the delay is run first. Then enter next state, in this case shooting.
- Explode  
  Self destroy and deal AOE damage.
- On death  
  Define if and if yes, which component is activated when enemy dies, for example explode.
- On spawn  
  Define component(s) / state(s) that are active when enemy spawns / the starting states.
- Splitting  
  Enemy is split into multiple, individual parts and only share some of their components. -> enemy has multiple hitboxes and respective hp pools, but same movement component - so the "move as one".
- The other kind of splitting  
  "Split" into multiple smaller enemies (play animation, spawn new enemies, destroy current enemy), for example on death. 
- On impact  
  When hitting a wall, do something.

## Environmental Mechanics
- Jumppad  
  Applies set amount of velocity to player. 
- One way platforms  
- Spikes  
- Turrets  
- Disappearing platforms  
  Disappear on set event, for example on contact with player (after short delay). 
- Basic moving platforms  
  Move along manually set path until reaching end of it, then pivot. 
- Pressure plates  
- Knockbackers  

## Weapons
