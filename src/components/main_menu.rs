use amethyst::{
    assets::Loader,
    ecs::Entity,
    prelude::*,
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};

#[derive(Default)]
pub struct MainMenu;

pub fn init_main_menu(world: &mut World) -> Vec<Entity>{
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let title_transform = UiTransform::new(
        "title".to_string(), // id
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        0.,   // x
        -50., // y
        1.,   // z
        500., // width
        50.,  // height
    );

    let title_entity = world
        .create_entity()
        .with(title_transform)
        .with(UiText::new(
            font.clone(),
            "Planetary Pong".to_string(),
            [1., 1., 1., 1.], // color
            50.,              // size
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let start_transform = UiTransform::new(
        "start".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        0.,
        -400.,
        1.,
        500.,
        25.,
    );

    let start_entity = world
        .create_entity()
        .with(start_transform)
        .with(UiText::new(
            font.clone(),
            "Select Number Of Players".to_string(),
            [1., 1., 1., 1.],
            25.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let zero_players_transform = UiTransform::new(
        "zero_players".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -30.,
        -500.,
        1.,
        15.,
        15.,
    );

    let zero_players_entity = world
        .create_entity()
        .with(zero_players_transform)
        .with(UiText::new(
            font.clone(),
            "zero".to_string(),
            [1., 1., 1., 1.],
            25.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let one_players_transform = UiTransform::new(
        "zero_players".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        0.,
        -500.,
        1.,
        15.,
        15.,
    );

    let one_players_entity = world
        .create_entity()
        .with(one_players_transform)
        .with(UiText::new(
            font.clone(),
            "one".to_string(),
            [1., 1., 1., 1.],
            25.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let two_players_transform = UiTransform::new(
        "two_players".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        30.,
        -500.,
        1.,
        15.,
        15.,
    );

    let two_players_entity = world
        .create_entity()
        .with(two_players_transform)
        .with(UiText::new(
            font.clone(),
            "two".to_string(),
            [1., 1., 1., 1.],
            25.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    vec!(
        start_entity,
        title_entity,
        zero_players_entity,
        one_players_entity,
        two_players_entity,
    )
}
