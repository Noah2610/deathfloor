
shooter
walker, ledge turn
shoot in set interval

patrol
walker, ledge turn
- when player enters LOS stop movement components, change hitbox and animation (prone), start shooting in set interval


aggro
- walker, ledge turn.
- when player enters LOS set higher movement speed and remove ledge and edge turn
- on death explode
- when hits wall explode


jumper
- walk
- when player los: stop movement components, after short delay jump towards player, loop. jump values based on current player pos


vertical shooter
- no gravity component
- vertical walker and wall pivot
- when player los start shooting in set interval

mini
- walk, very fast, no ledge detect
- explode on player collision or after set delay (randomized delay?)

security
- walk
- when player los spawn 3 minis in short interval, stop walking, start shooting. loop shooting until player exits los then walk. loop


flying kamikaze
- "walk", only wall turn.
- no gravity
- when player los (large los zone) start chasing player (like stabman ghosts) and explode after short delay
- exlode on player collision

riccochet tank
- slow walk, ledge and wall turn
- when player los stop moving and shoot riccochet bullet diagonally upwards in player direction on x axis

riccochet bullet
- basically rubber ball 
- deal player damage on contact and despawn


walking tower (3)
- idle, no walking
- on player los move slowly and randomize between 3 fire points - depending on what the randomizer hits the tower either shoots
from its top, its middle part or its bottom. (in player x direction). loop

walking tower (4)
- idle, no walking
- on player los move slowly and randomize between 4 fire points but 2 times. - shoots a bullet from 2 random positions. loop

- rocket bullet
move forward with slightly randomized y
explode on wall or player collision

walking tower (2)
- walk
- on player los move slower and shoot a rocket bullet from both fire points. loop

sleeping tower
- really strong
- idle until being hit by any projectile (player or enemy)
- then: slowly walk towards player, shoot rocket from randomized fire points in short interval

flying shooter
- no gravity
- move to randomized locations in near proximity. when there, shoot into player direction on x axis, after short delay loop


