use amethyst::{
    assets::{AssetStorage, Handle, Loader, PrefabLoader, PrefabLoaderSystemDesc, Processor, RonFormat},
    audio::AudioSink,
    core::{timing::Time, transform::Transform, ArcThreadPool},
    ecs::{Dispatcher, DispatcherBuilder, Entity},
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{
        Anchor, RenderUi, UiBundle, UiButtonBuilder, UiCreator, UiEvent, UiFinder, UiImage, UiText,
    },
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::audio::init_audio;
use crate::components::ball::init_ball;
use crate::components::paddle::init_paddles;
use crate::components::scoreboard::init_scoreboard;
use crate::components::main_menu::init_main_menu;

use crate::systems;

pub const ARENA_HEIGHT: f32 = 200.0;
pub const ARENA_WIDTH: f32 = 200.0;

#[derive(Default)]
pub struct MainMenuState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
    entities: Vec<Entity>,
}
impl<'a, 'b> SimpleState for MainMenuState<'a, 'b> {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let world = &mut data.world;
        let dispatcher_builder = DispatcherBuilder::new();

        // reuse main thread pool
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/main_menu.ron", ());
        });

        init_audio(world);
        init_camera(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
       let _ = data.world.delete_entities(&self.entities);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event{
            StateEvent::Window(win_event) => {
                if is_key_down(&win_event, VirtualKeyCode::Return) {
                    println!("State change -> GameplayState");
                    Trans::Replace(Box::new(GameplayState::default()))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                println!("UI Event {:?}", ui_event);
                println!("UI Id {:?}", ui_event.target.id());
                println!("UI Gen {:?}", ui_event.target.gen());
                Trans::None
            }
            StateEvent::Input(input_event) => {
                Trans::None
            }
        }
    }
}

#[derive(Default)]
pub struct GameplayState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    ball_spawn_timer: Option<f32>,
    num_players: u8,
}
impl<'a, 'b> SimpleState for GameplayState<'a, 'b> {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let world = &mut data.world;

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(systems::PaddleSystem, systems::PADDLE_SYSTEM, &[]);
        dispatcher_builder.add(systems::BallMoveSystem, systems::BALL_MOVE_SYSTEM, &[]);
        dispatcher_builder.add(
            systems::BounceSystem,
            "collision_system",
            &[systems::PADDLE_SYSTEM, systems::BALL_MOVE_SYSTEM,],
        );
        dispatcher_builder.add(systems::WinnerSystem, "winner_system", &["ball_system"]);
        dispatcher_builder.add(systems::AIBigBrainSystem, "ai_big_brain_system", &[]);
        dispatcher_builder.add(
            systems::PaddleMoveSystem,
            systems::PADDLE_MOVE_SYSTEM,
            &[systems::PADDLE_SYSTEM],
        );

        // reuse main thread pool
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
        let sprite_sheet_handle = load_sprite_sheet(world);
        self.sprite_sheet_handle
            .replace(sprite_sheet_handle.clone());
        init_paddles(world, self.sprite_sheet_handle.clone().unwrap(), self.num_players);
        init_scoreboard(world);

        if let Some(mut timer) = self.ball_spawn_timer.take() {
            {
                let time = data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }
            if timer <= 0.0 {
                init_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
            } else {
                self.ball_spawn_timer.replace(timer);
            }
        }
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.read_resource::<AudioSink>().play();
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PauseState));
            }
        }
        Trans::None
    }
}

struct PauseState;
impl SimpleState for PauseState {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.read_resource::<AudioSink>().pause();
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }
        Trans::None
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
