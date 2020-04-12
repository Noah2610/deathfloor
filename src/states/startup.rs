use super::state_prelude::*;
use crate::helpers::resource;
use std::path::PathBuf;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        register_components(data.world);
        load_spritesheets(&mut data.world);
        load_audio(&mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_core(data.world);
        Trans::Switch(Box::new(LoadIngame::default()))
    }
}

fn register_components(world: &mut World) {
    world.register::<Player>();
    world.register::<Camera>();
    world.register::<Shooter>();
    world.register::<Jumppad>();
    world.register::<JumppadAffected>();
    world.register::<Enemy>();
    world.register::<Lifecycle>();
}

fn load_spritesheets(world: &mut World) {
    let mut sprite_sheet_handles =
        world.write_resource::<SpriteSheetHandles<PathBuf>>();
    sprite_sheet_handles
        .load(resource("spritesheets/player_bullet.png"), world);
    sprite_sheet_handles.load(resource("spritesheets/colors.png"), world);
}

fn load_audio(world: &mut World) {
    let audio_settings =
        world.read_resource::<Settings>().general.audio.clone();

    {
        let mut sounds = Sounds::default();
        let mut load_sound = |sound_type: SoundType| -> Result<(), String> {
            let path = sound_type.path();
            sounds.load_audio(
                sound_type,
                path,
                &world.read_resource(),
                &world.read_resource(),
            )
        };
        load_sound(SoundType::Jump).unwrap();
        load_sound(SoundType::Shoot).unwrap();
        world.insert(sounds);
    }

    {
        let mut songs = Songs::default()
            .with_playback_order(vec![SongType::Cntrlgun, SongType::Floor1])
            .with_playback_state(SongPlaybackState::Playing)
            .with_playback_behavior(SongPlaybackBehavior::Autoplay)
            .with_volume(audio_settings.songs_volume);
        let mut load_song = |song_type: SongType| -> Result<(), String> {
            let path = song_type.path();
            songs.load_audio(
                song_type,
                path,
                &world.read_resource(),
                &world.read_resource(),
            )
        };
        load_song(SongType::Cntrlgun).unwrap();
        load_song(SongType::Floor1).unwrap();
        world.insert(songs);
    }
}
