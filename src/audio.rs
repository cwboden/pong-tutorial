use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{AudioSink, output::Output, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};
use std::{iter::Cycle, vec::IntoIter};

const BOUNCE_SOUND_PATH: &str = "audio/bounce.ogg";
const SCORE_SOUND_PATH: &str = "audio/score.ogg";

const MUSIC_TRACK_PATHS: &[&str] = &[
    "audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];

pub struct Sounds {
    pub bounce_sfx: SourceHandle,
    pub score_sfx: SourceHandle,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn initialize_audio(world: &mut World) {
    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25); // Reduce the volume of the music

        let sounds = Sounds {
            bounce_sfx: load_audio_track(&loader, &world, BOUNCE_SOUND_PATH),
            score_sfx: load_audio_track(&loader, &world, SCORE_SOUND_PATH),
        };

        let music = Music {
            music: MUSIC_TRACK_PATHS
                .iter()
                .map(|file| load_audio_track(&loader, &world, file))
                .collect::<Vec<_>>()
                .into_iter()
                .cycle(),
        };

        (sounds, music)
    };

    // Insert in separate scope -- World objects don't allow insertion of new resources while
    // `Loader` is borrowed.
    world.insert(sound_effects);
    world.insert(music);
}

pub fn play_bounce_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_score_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.score_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}
