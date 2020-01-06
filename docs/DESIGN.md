# Design notes
## The player
The player can move left/right and jump.

## The deathfloor
The _deathfloor_ moves upwards at a constant speed.  
Its position is independent of the player or the camera.  
When the player touches it, it's game over.

## The camera
The camera follows the player.

## Platforms
- __Default platform block__  
  For now, there should only be the default platform block,  
  which the player uses to move and jump on.

## Obstacles
- __Spikes__  
  How exactly the spikes will work is undecided. Possibilities:  
  - Kills the player on touch
  - Stops/freezes the player for a couple seconds,  
    to allow the deathfloor to catch up.
