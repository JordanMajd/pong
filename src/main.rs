use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::{
         frame_limiter::FrameRateLimitStrategy, transform::TransformBundle,
        HideHierarchySystemDesc,
    },
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod audio;
mod components;
mod event;
mod game_data;
mod pong;
mod systems;

use crate::audio::Music;
use crate::pong::LoadingState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir().unwrap();
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)
        .unwrap();

    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)
                .unwrap()
                .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default());

    let nice_game_data = game_data::NiceGameDataBuilder::default()
        .with_bundle(rendering_bundle)
        .with_bundle(input_bundle)
        .with_bundle(TransformBundle::new())
        .with_bundle(AudioBundle::default())
        .with_bundle(UiBundle::<StringBindings>::new())
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with_system_desc(HideHierarchySystemDesc::default(), "hide_hierarchy", &[]);

    let assets_dir = app_root.join("assets");
    let mut game = CoreApplication::<
        crate::game_data::NiceGameData,
        event::GameEvent,
        event::GameEventReader,
    >::new(assets_dir, LoadingState::default(), nice_game_data)?;
    game.run();

    Ok(())
}
