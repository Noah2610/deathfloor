use super::system_prelude::*;
use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output as AudioOutput;
use amethyst::audio::Source;
use std::ops::Deref;

const DEFAULT_VOLUME: f32 = 1.0;

#[derive(Default)]
pub struct HandleSoundPlayersSystem;

impl<'a> System<'a> for HandleSoundPlayersSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, SoundPlayer>,
        ReadExpect<'a, Sounds>,
        Read<'a, AssetStorage<Source>>,
        Read<'a, AudioOutput>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut sound_player_store,
            sounds,
            asset_storage,
            audio_output,
        ): Self::SystemData,
    ) {
        for (entity, sound_player) in
            (&entities, &mut sound_player_store).join()
        {
            for action in sound_player.drain_actions() {
                match action {
                    SoundAction::Play(sound_type) => {
                        play_sound(
                            &sounds,
                            &asset_storage,
                            &audio_output,
                            &sound_type,
                            DEFAULT_VOLUME,
                        );
                    }
                    SoundAction::PlayWithVolume(sound_type, volume) => {
                        play_sound(
                            &sounds,
                            &asset_storage,
                            &audio_output,
                            &sound_type,
                            volume,
                        );
                    }
                }
            }
        }
    }
}

fn play_sound(
    sounds: &Sounds,
    asset_storage: &AssetStorage<Source>,
    audio_output: &AudioOutput,
    sound_type: &SoundType,
    volume: f32,
) {
    if let Some(sound_source) = sounds.get_handle(sound_type) {
        if let Some(sound) = asset_storage.get(sound_source) {
            audio_output.play_once(sound, volume);
        } else {
            eprintln!(
                "[WARNING]\n    Sound source for SoundType {:?} is not loaded \
                 in asset storage",
                sound_type
            );
        }
    } else {
        eprintln!(
            "[WARNING]\n    Sound source for SoundType {:?} is not registered",
            sound_type
        );
    }
}
