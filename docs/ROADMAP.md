# Rough roadmap
## v0.0.1
- [x] __Map loading__  
  Create maps in _tiled_ which can be loaded in-game  
- [x] __Player__ with basic movement (left/right/jump)
- [x] __Camera__ that follows the player

## v0.0.2
- __PIVOT__
- [x] way to reload level when ingame  
  with `debug` feature (use `bin/run` script), press `R` to reload current level
- [x] basic walljump ([#7])
- [x] basic jumppad ([#8])
- [x] restrain camera within level borders

## v0.0.3
- [x] enemy types, composable entity features
- [x] player shooting ([#9])
- [x] enemies ([#10])
- [x] health system ([#11])

## v0.0.4
- [ ] menus ([#24])
  - [ ] main menu
  - [ ] paused menu
- [ ] audio ([#23])
  - [x] SFX
  - [ ] BGM
- [ ] extended tile configs  
  tile animations ([#20]), tile events/actions

## v0.0.5
- features:
- [ ] basic pickup system
- [ ] moving platforms
- [ ] "LOS" enemy component 
- [ ] camera system
- [ ] ability select screen with functional UI

- audio:
- [ ] music, implement test tracks
- [ ] sfx:
- [ ] shooting, jumping
- [ ] footsteps? different footsteps for different ground? (ID set as property in tiled?)
- [ ] enemy sounds

- enemies:
- [ ] configure more enemies
- [ ] implement enemies into playable levels
- [ ] more enemy sprites and animations

- levels:
- [ ] more playable levels with up to date mechanics and enemies

- environment graphics:
- [ ] more backgrounds
- [ ] animated decoration and background elements (grass? etc..)
- [ ] polish tilesets, add animated foreground tiles

[#7]: https://github.com/Noah2610/deathfloor/issues/7
[#8]: https://github.com/Noah2610/deathfloor/issues/8
[#9]: https://github.com/Noah2610/deathfloor/issues/9
[#10]: https://github.com/Noah2610/deathfloor/issues/10
[#11]: https://github.com/Noah2610/deathfloor/issues/11
[#20]: https://github.com/Noah2610/deathfloor/issues/20
[#23]: https://github.com/Noah2610/deathfloor/issues/23
[#24]: https://github.com/Noah2610/deathfloor/issues/24
