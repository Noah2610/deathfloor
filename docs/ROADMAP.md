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
  - [x] main menu
  - [ ] paused menu
- [x] audio ([#23])
  - [x] SFX
  - [x] BGM
- [x] extended tile configs  
  tile animations ([#20]), tile events/actions

## v0.0.5
### Features
- [ ] ability system ([#52])
- [ ] consumable pickup system ([#49])
- [ ] moving platforms ([#50])
- [ ] line-of-sight detection ([#51])
- [ ] camera system ([#19])
- [ ] footsteps ([#53])

### Audio
- [ ] music, implement test tracks
- [ ] shooting, jumping
- [ ] footsteps
- [ ] enemy sounds

### Enemies
- [ ] refactor existing enemy configs ([#48])
- [ ] configure more enemies
- [ ] implement enemies into playable levels
- [ ] more enemy sprites and animations

### Levels
- [ ] more playable levels with up to date mechanics and enemies

### Environment Graphics
- [ ] more backgrounds
- [ ] animated decoration and background elements (grass? etc..)
- [ ] polish tilesets, add animated foreground tiles

[#7]: https://github.com/Noah2610/deathfloor/issues/7
[#8]: https://github.com/Noah2610/deathfloor/issues/8
[#9]: https://github.com/Noah2610/deathfloor/issues/9
[#10]: https://github.com/Noah2610/deathfloor/issues/10
[#11]: https://github.com/Noah2610/deathfloor/issues/11
[#19]: https://github.com/Noah2610/deathfloor/issues/19
[#20]: https://github.com/Noah2610/deathfloor/issues/20
[#23]: https://github.com/Noah2610/deathfloor/issues/23
[#24]: https://github.com/Noah2610/deathfloor/issues/24
[#48]: https://github.com/Noah2610/deathfloor/issues/48
[#49]: https://github.com/Noah2610/deathfloor/issues/49
[#50]: https://github.com/Noah2610/deathfloor/issues/50
[#51]: https://github.com/Noah2610/deathfloor/issues/51
[#52]: https://github.com/Noah2610/deathfloor/issues/52
[#53]: https://github.com/Noah2610/deathfloor/issues/53
