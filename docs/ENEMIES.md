# shooter
- walker, ledge and wall turn
- shoot in set interval
- medium hp

# charger
- walker, ledge and wall turn
- when player los: change animation, increase speed, remove ledge turn component
- damage player on contact
- collision with solids, spikes, jumppads, player
- low hp

# patrol
- walker, ledge turn
- when player enters LOS stop movement components,  
  change hitbox and animation (prone), start shooting in set interval
- medium hp

# kamikaze
- walker, ledge and wall turn.
- when player enters LOS set higher movement speed and remove ledge and edge turn
- on death explode
- when hits wall explode
- low hp

# jumper
- walk
- when player los:  
  stop movement components, after short delay jump towards player, loop.  
  jump values based on current player pos
- medium hp

# vertical shooter
- no gravity component
- vertical walker and wall pivot
- when player los start shooting in set interval
- medium hp

# mech
- walk, but: walk consists of 2 states? / variants? / intervals:  
  Move forward and play animation for short interval, stop movement and animation for short interval, loop
-> because:

# mini
- walk, very fast, no ledge detect
- explode on player collision or after set delay (randomized delay?)
- low hp

# security
- walk
- when player los spawn 3 minis in short interval, stop walking, start shooting.  
  loop shooting until player exits los then walk. loop
- medium / high hp?

# mine
- "walk", only wall turn.
- no gravity
- exlode on player collision

# flying kamikaze
- "walk", only wall turn.
- no gravity
- when player los (large los zone) start chasing player (like stabman ghosts) and explode after short delay
- exlode on player collision
- low hp

# riccochet tank
- slow walk, ledge and wall turn
- when player los stop moving and shoot riccochet bullet diagonally upwards in player direction on x axis
- high hp

# riccochet bullet (projectile)
- basically rubber ball
- deal player damage on contact and despawn

# walking tower (3)
- idle, no walking
- on player los move slowly and randomize between 3 fire points - depending on what the randomizer hits the tower either shoots
from its top, its middle part or its bottom. (in player x direction). loop

# walking tower (4)
- idle, no walking
- on player los move slowly and randomize between 4 fire points but 2 times. - shoots a bullet from 2 random positions. loop

# rocket bullet (projectile)
move forward with slightly randomized y  
explode on wall or player collision

# walking tower (2)
- walk
- on player los move slower and shoot a rocket bullet from both fire points. loop

# sleeping tower
- really strong
- idle until being hit by any projectile (player or enemy)
- then: slowly walk towards player, shoot rocket from randomized fire points in short interval
- high hp

# flying shooter
- no gravity
- move to randomized locations in near proximity. when there, shoot into player direction on x axis, after short delay loop

# bomb (projectile)
- explode on contact with player or solids
- gravity affected?

# bomber
- no gravity, walker, wall turn
- "drop bombs" in set interval -> either have bombs be regular projectile with const velocity or, preferably, have them be gravity affected and thus accalerate after being spawned
- bombs explode on contact with player or solids

# dropper
- no gravity, no movement components, placed at ceiling
- when player enters los (below dropper), add gravity component (and maybe minor -y vel in case gravity alone is not sufficient for the enemy to drop fast enough
- on contact damage player
- maybe: explode on floor contact
- maybe: switch to variant with movement components on floor contact
- low hp

# tank
- gravity, walker, ledge and wall turn
- on player los: stop moving, set lower friction values, start shooting in set interval. everytime the enemy shoots he applies x knockback to himself (opposite to shooting direction)
- high hp

# bomb dropper


# flying platform


# flying platform shooter


# discharger
- gravity, walker, ledge and wall turn
- spawn 2 discharges in set interval, one to the left and one to the right
- medium hp

# discharge (projectile)
- gravity affected
- gets spawned with slight x and y "impulse" ("jump") (values dependent on variant, variant dependent on information given by the enemy who spawns the discharge)
- after short delay (which is used to play a spawn animation) switch to another variant with different animation and movement components, walker and wall turn
- damage player on contact
- destroy on player contact or after 10 sec 

# jumping battery
- gravity, no movement components
- on player LOS: jump with hardcoded y and semi randomized x value. when touching floor again, spawn 2 discharges, one to the left and one to the right. short delay, loop.
- high hp

# electric barrier
- no gravity, no movement, doesnt take damage 
- sprite consists of 2 coils inbetween which electric static is drawn
- switch between 2 states in set interval
- state 1: no static, no damage, maybe
- state 2: animated static, damage on contact

# jumping shooter
- gravity, no movement components
- on player LOS, jump with hardcoded y and x (x sign dependent on player pos but value is set) and shoot 3 projectiles into facing direction in short interval. short delay after landing, then loop.

# basic turret

# burst turret

# shotgun turret

# peeker
- gravity, no movement
- switches between 2 variants (animation, (in)vincibility, shooting) in set interval

# walking peeker
- peeker except he walks forward during shooting variant

# shocker
- gravity, walker, ledge and wall turn
- on player LOS: increase movement speed, spawn electric static in front of it in set interval

# electric static:
- large "projectile" that damages player on contact and has short lifetime
- has walker component but no set movement values - values are passed by entity that spawns the static (does / can it work like that?) so the static can, when spawned by a turret, be "stationary" but, when spawned by a moving enemy, "move with" the enemy (at the enemys vel) and be projected in front of the enemy like a flamethrower.

# diagonal shooter

# birdman