use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    audio::{output::Output, AudioSink, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};
use std::{iter::Cycle, vec::IntoIter};

const BOUNCE_SOUND: &str = "audio/bounce.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";

const MUSIC_TRACKS: &[&str] = &[
    "audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_track(
    loader: &Loader,
    progress: &mut ProgressCounter,
    world: &World,
    file: &str,
) -> SourceHandle {
    loader.load(file, OggFormat, progress, &world.read_resource())
}

pub fn init_audio(world: &mut World, progress: &mut ProgressCounter) {
    let (sound_effect, music) = {
        let loader = world.read_resource::<Loader>();
        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25);

        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, progress, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };

        let sound = Sounds {
            bounce_sfx: load_audio_track(&loader, progress, &world, BOUNCE_SOUND),
            score_sfx: load_audio_track(&loader, progress, &world, SCORE_SOUND),
        };
        (sound, music)
    };

    world.insert(sound_effect);
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
