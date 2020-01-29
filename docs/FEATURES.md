Death Floor V3 early feature list (roughly descending in priority / expected effort)

- Basic jumppad

- Basic spikes

- Basic stun projectiles: Upon impact input is locked. If contact with a projectile happens while player is mid air, 
  current velocity / momentum carries over. Player input remains locked until player touches a floor (not any solid but only a floor - collision + y velocity check?).
  If contact is made while player is grounded, input is locked and minor x and y velocity is given (x dependent on direction of impact). Input remains locked
  until player touches floor again. 

- Basic turrets: Environmental enteties that dispense projectiles with a fixed fire rate in a fixed direction (FR and direction can be set in tiled).

- LOS system for turrets and other turret-like mechanics: System that checks if player is located in a manually set LOS (raycasts?). 
  
  Basic use example: Turrets - when player enters LOS, a projectile is being fired. (If player is (still or again) located in LOS when set fire rate cooldown has expired
  another projectile is being fired, rinse and repeat)
  
  Advanced use example: Icicles - 2 points from which LOS is being checked (in a set angle / direction) are set slightly to the right and to the left of an Icicle object that is static 
  until LOS is (one of them is) positively checked - if that happens, the icicle falls. (the falling icicle (which should, optimally, be gravity affected and not just travel 
  at a set speed like a projectile) is a different system entirely but should, given the physics engines capabilities, be within the realms of possibility).   

- Basic shop / powerup system: Powerup / shop elements can be placed in tiled and given defining properties (what item is being "sold" / picked up, how much does it cost, etc...)
  and accessed ingame. Also requires a basic coin / currency system to be in place. 

- Advanced movement mechanics: Directional air-dash and additional movement tech to master, for example: Execute a walljump with perfect, tight timing
  to gain additional vel / range (Accompanied by a distinct animation + maybe sound).

- Ice: Solids that don't instantly kill velocity on contact but slowly (an exponentional reduction would probably be the best fit) reduce momentum. 
  
- Moving solids in complex shapes: Moving "platforms" consisting of multiple geometric shapes, for example an "L"- shaped platform. 

- Customizable tiled properties for enviromental mechanics: 
  
  - Paths for moving solids: As much control as possible over how paths can be layed out (for example chaining multiple horizontal and vertical paths of varying length together 
    in a specific sequence)
  
  - Timing / Offset for turrets and moving solids: Given that turrets start shooting (in their fixed FR) and moving platforms start moving as soon as they are loaded,
    some degree of unpredictability (for example due to lag) and other problems could occur. A potential approach would be an advanced loading system
    that "queues" objects close together when loaded and only starts executing their logic when all objects that belong to their respective "group" are loaded and ready. Those groups would 
    be set manually, for example through a custom tiled property that takes the ID's of all objects we want to be initialized simultaneously as values. 
    Note: Such a system may seem like overkill (and I don't know if this specific approach makes sense or if it would even be possible since i dont know how exactly the loading system works)
    but any solution, even if time consuming, would definitely be worth it (but has of course a rather low priority and doesn't need to be in place any time soon). 
    
    To enable precise and complex level design (have more control over how interweaved cycles
    can be designed) a feature along the lines of an "offset" would be great. The offset could be set as a property in tiled, and defines how much time has to pass after the object the
    offset is applied to starts executing its logic -> in case of a turret, starts shooting. For such a system to make sense, a system to norm initialization times like the one described
    above would be required.

- Basic enemy system: Basic grounded "roamer" enemies that stun the player on contact and simply trace left and right until they either hit a wall or encounter a ledge.