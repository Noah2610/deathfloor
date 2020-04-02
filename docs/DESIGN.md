# Design notes

<details>
<summary>Table of Contents</summary>

- [General](#general)
  - [Player Movement](#player-movement)
  - [Environmental Animations](#environmental-animations)
- [Combat](#combat)
  - [Health](#health)
  - [I-frames](#i-frames)
  - [Player death](#player-death)
- [Enemies ](#enemies)
  - [Components](#components)
    - [Planned](#planned)
      - [`Spikey`](#spikey)
      - [`Walker`](#walker)
      - [`ForwardShooter`](#forwardshooter)
      - [`Chaser`](#chaser)
      - [`Jumppad`](#jumppad)
      - [`AnimationsContainer`](#animationscontainer)
    - [To plan (TODO)](#to-plan-todo)
  - [Events](#events)
    - [`OnSpawn`](#onspawn)
    - [`OnDeath`](#ondeath)
    - [`OnCollision`](#oncollision)
    - [`Interval`](#interval)
  - [Actions](#actions)
    - [`Delay`](#delay)
    - [`Group`](#group)
    - [`Explode`](#explode)
    - [`Drop`](#drop)
    - [`Random`](#random)
    - [`Jump`](#jump)
- [Environmental Mechanics](#environmental-mechanics)
- [Weapons](#weapons)
  - [Weapon types](#weapon-types)
- [SFX](#sfx)
- [Level setting / theme ideas](#level-setting--theme-ideas)

</details>

## General

- Level reload key
- Level reset at death / falling out of level
- Basic level manager
- Next-room-object to be placed in tiled
- Room to room traversal:
  After a short transition teleport player to new room. new room is seperate tmx / json, so different rooms can have different room sizes / camera settings / zoom level?

### Player mechanics (movement and weaponary) per level
- Pitch:
When a new level is unlocked, the player is restricted to certain movement and weapon mechanics to enable "precise and concrete" level design. However, when the player has beaten the level,
it can be replayed with the movement and weapon mechanics of other, already beaten levels. - beating levels globally unlocks their respective mechanics for other, already beaten levels. 
I would imagine this system to be built upon "slots" - each level gives the player a set (?) amount of available weapon and movement slots that are filled with certain mechanics and can, after
initially beating the level, be filled by the player with mechanics of their choice.
Such a system would enable high replayability and gameplay diversity by pretty much only utilizing already existent mechanics -> ressource efficent. The only completely new system would be the
interactable slot UI. 


- Camera
  Limited by level borders

- Player hitboxes:
  Seperate hitboxes for solids and damage 

### Player Movement
- Jump 
- Walljump
- Slide:
  Change hitbox and gain static velocity for duration of slide. spammable?
- Be able to limit "jump_gravity" active duration to set amount of time after using jump or to when player has positive y velocity (so when he's falling it doesnt feel like everything is underwater).

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
Enemies consist of various components that can be combined to craft simple behavior.

#### Planned
Components, whose details have been thought about, and which can be worked on.

##### `Spikey`
- [ ] implemented
```
spikey: (
    damage: 123, // amount of damage to deal
),
```
Damage on collision:  
Deals damage to the player on collision.

##### `Walker`
- [x] implemented
```
walker: (
    // Velocity to _set_ each frame
    x: 10.0,
    y: None,
),
```
Basic roaming:  
Move left and right (initial direction can be set in tiled)  
~~until hitting either a ledge or a wall, then pivot~~  
(pivot on solid collision should be done by an _event/action_ (_TODO_)).

##### `ForwardShooter`
__TODO:__ _Figure out proper name_
- [ ] implemented
```
forward_shooter: (
    interval_ms: 5000, // shoot interval in milliseconds
),
```
Basic shooting:  
Spawns projectiles in walking direction in a set interval.

##### `Chaser`
- [ ] implemented
```
chaser: (
    // Distance to player, before it starts chasing (x, y)
    distance: (300.0, 150.0),
),
```
__NOTE__  
Let's ignore LOS for now, and just work with _distance to player_.  
I still need to figure out how to do LOS properly.

Basic chasing:  
When player enters LOS enemy moves towards player (flying),  
when player leaves LOS they freeze (until Player reenters LOS)

##### `Jumppad`
- [x] implemented
```
jumppad: (
    // Jumppad strength (x, y)
    // Both values are optional (use `None` to omit)
    strength: (None, 400.0),
),
```
Player can jump off of them

##### `AnimationsContainer`
- [x] implemented
```
animations: {
    // TODO: Proper `AnimationsContainer` documentation
    Idle: Cycle([
        (0, 100),
        (1, 100),
        (2, 100),
    ]),
    Walk: Cycle([
        (3, 100),
        (4, 100),
        (5, 100),
    ]),
},
```

#### To plan (TODO)
These components are more complicated and require more thought,  
or are low-priority. These still need to be planned out.

<details>
<summary>Components</summary>

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
</details>

### Events
Add enemy events to enemy configs in their `events` field.  
Each event can trigger multiple _action_.  
An _action_ can do arbitrary stuff to an enemy's components.  
See the section about [actions](#actions) for details.

#### `OnSpawn`
- [x] implemented
```
OnSpawn: <action>,
```
Triggers _action_ when the enemy spawns / is first loaded.

#### `OnDeath`
- [ ] implemented
```
OnDeath: <action>,
```
Triggers _action_ when the enemy dies.

#### `OnCollision`
- [x] implemented
```
// Takes optional collision QueryExpression.
// If given, will only trigger action if
// FindQuery with given QueryExpression matches.
// TODO: QueryExpresion documentation.
OnCollision(<query>): <action>,
```
Triggers _action_ on collision with any _collidable_, that this  
_collider_ can collide with.  
Optionally, pass a [`QueryExpression`](https://github.com/Noah2610/deathframe/blob/develop/deathframe_physics/src/query/exp.rs#L20) (used with [`FindQuery`](https://github.com/Noah2610/deathframe/blob/develop/deathframe_physics/src/query/find_query.rs));  
if given, will only trigger action if query matches.

#### `Interval`
- [ ] implemented
```
// Takes interval delay in milliseconds.
Interval(<delay_ms>): <action>,
```
Triggers _action_ in regular intervals.

### Actions
Actions are triggered by [_events_](#events).

#### `Delay`
- [ ] implemented
```
<event>: Delay(
    delay_ms: 2000, // time in milliseconds to wait before triggering action
    action: ...,
),
```
When this action triggers, waits for `delay_ms` milliseconds,  
before triggering its `action`.

#### `Group`
- [x] implemented
```
<event>: Group([
    <action0>,
    <action1>,
    <action2>,
    ...
]),
```
Groups multiple actions into one.  
An event can trigger multiple actions at once  
using this `Group` action.

#### `Explode`
- [ ] implemented
```
<event>: Explode(
    damage: 420,
    radius: 69,
),
```
Self destroy and deals of AOE `damage` in the given `radius`.

#### `Drop`
- [ ] implemented
```
<event>: Drop( ??? ),
```
__TODO: items__  
Drop something, for example on death drop health pack.

#### `Random`
- [ ] implemented
```
<event>: Random(
    chance: 0.5,
    action: <action>,
),
```
When triggered, will randomly _succeed_ or _fail_, depending on the given `chance`  
value, which is a number between `0.0` and `1.0` (both inclusive).  
If successful, triggers its `action`.  

<small>__NOTE:__ Maybe introduce another layer: `Expression`... `:)`</small>

#### `SetVelocity`
- [x] implemented
```
<event>: SetVelocity(
    x: None,
    y: 400.0,
),
```
Sets velocity to the given `x`, `y` values.  
Can be used to make the enemy jump, for example.
Both velocities are optional.

#### `Echo`
- [x] implemented
```
<event>: Echo(<message_to_print>),
```
Prints the given message string to the console.  
(For debugging / development).

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
- Water:
  "Field" that has texture and different gravity settings.
- Ice:
- Explosive boxes:
  Explode when afflicted with any kind of damage (from any source)  
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

## Level setting / theme ideas
