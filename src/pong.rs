use amethyst::{
    assets::{AssetStorage, Completion, Handle, Loader, ProgressCounter},
    audio::AudioSink,
    core::{timing::Time, transform::Transform, ArcThreadPool, HiddenPropagate},
    ecs::{Dispatcher, DispatcherBuilder, Entity},
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{UiCreator, UiEvent, UiEventType, UiFinder, UiLoader},
};

use crate::audio::init_audio;
use crate::components::ball::init_ball;
use crate::components::paddle::init_paddles;
// use crate::components::scoreboard::ScoreBoard;
use crate::event::GameEvent;
use crate::game_data::NiceGameData;
use crate::systems;

pub const ARENA_HEIGHT: f32 = 200.0;
pub const ARENA_WIDTH: f32 = 200.0;

#[derive(Default)]
pub struct LoadingState {
    pub progress_counter: ProgressCounter,
}

impl<'a, 'b> State<NiceGameData<'a, 'b>, GameEvent> for LoadingState {
    fn on_start(&mut self, data: StateData<'_, NiceGameData<'a, 'b>>) {
        data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/main_menu.ron", &mut self.progress_counter)
        });
        let handle = data.world.exec(|loader: UiLoader<'_>| {
            loader.load("ui/scoreboard.ron", &mut self.progress_counter)
        });
        data.world.insert(handle);
        init_audio(data.world, &mut self.progress_counter);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, NiceGameData<'a, 'b>>,
        _event: GameEvent,
    ) -> Trans<NiceGameData<'a, 'b>, GameEvent> {
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, NiceGameData<'a, 'b>>,
    ) -> Trans<NiceGameData<'a, 'b>, GameEvent> {
        data.data.update(data.world);
        match self.progress_counter.complete() {
            Completion::Complete => Trans::Switch(Box::new(MainMenuState::default())),
            Completion::Failed => Trans::Quit,
            Completion::Loading => Trans::None,
        }
    }
}

#[derive(Default)]
pub struct MainMenuState<'x, 'y> {
    dispatcher: Option<Dispatcher<'x, 'y>>,
    entities: Vec<Entity>,
    zero_player_button: Option<Entity>,
    one_player_button: Option<Entity>,
    two_player_button: Option<Entity>,
    settings_button: Option<Entity>,
}

impl<'a, 'b> State<NiceGameData<'a, 'b>, GameEvent> for MainMenuState<'_, '_> {
    fn on_start(&mut self, mut data: StateData<'_, NiceGameData<'_, '_>>) {
        let world = &mut data.world;
        let dispatcher_builder = DispatcherBuilder::new();
        // reuse main thread pool
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

        world.exec(|ui_finder: UiFinder<'_>| {
            self.zero_player_button = ui_finder.find("zero_player_button");
            self.one_player_button = ui_finder.find("one_player_button");
            self.two_player_button = ui_finder.find("two_player_button");
            self.settings_button = ui_finder.find("settings_button");
        });

        init_camera(world);
    }

    fn on_stop(&mut self, data: StateData<'_, NiceGameData<'_, '_>>) {
        let _ = data.world.delete_entities(&self.entities);
    }

    fn update(
        &mut self,
        data: StateData<'_, NiceGameData<'a, 'b>>,
    ) -> Trans<NiceGameData<'a, 'b>, GameEvent> {
        data.data.update(data.world);
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, NiceGameData<'a, 'b>>,
        event: GameEvent,
    ) -> Trans<NiceGameData<'a, 'b>, GameEvent> {
        match event {
            GameEvent::Window(win_event) => {
                if is_key_down(&win_event, VirtualKeyCode::Return) {
                    transiton_game_state(data.world, 0)
                } else {
                    Trans::None
                }
            }
            GameEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.zero_player_button {
                    transiton_game_state(data.world, 0)
                } else if Some(target) == self.one_player_button {
                    transiton_game_state(data.world, 1)
                } else if Some(target) == self.two_player_button {
                    transiton_game_state(data.world, 2)
                } else if Some(target) == self.settings_button {
                    Trans::None
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }
}

#[derive(Default)]
pub struct GameplayState<'x, 'y> {
    dispatcher: Option<Dispatcher<'x, 'y>>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    ball_spawn_timer: Option<f32>,
    num_players: u8,
}
impl<'a, 'b> State<NiceGameData<'a, 'b>, GameEvent> for GameplayState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, NiceGameData<'a, 'b>>) {
        let world = data.world;

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(systems::PaddleSystem, systems::PADDLE_SYSTEM, &[]);
        dispatcher_builder.add(systems::BallMoveSystem, systems::BALL_MOVE_SYSTEM, &[]);
        dispatcher_builder.add(
            systems::BounceSystem,
            "collision_system",
            &[systems::PADDLE_SYSTEM, systems::BALL_MOVE_SYSTEM],
        );
        dispatcher_builder.add(
            systems::ScoreSystem,
            systems::SCORE_SYSTEM,
            &[systems::BALL_MOVE_SYSTEM],
        );
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
        init_paddles(
            world,
            self.sprite_sheet_handle.clone().unwrap(),
            self.num_players,
        );
        self.ball_spawn_timer.replace(1.0);

        // world.create_entity().with(ScoreBoard::new()).build();
    }

    fn on_resume(&mut self, data: StateData<'_, NiceGameData<'a, 'b>>) {
        data.world.read_resource::<AudioSink>().play();
    }

    fn update(
        &mut self,
        data: StateData<'_, NiceGameData<'a, 'b>>,
    ) -> Trans<NiceGameData<'a, 'b>, GameEvent> {
        data.data.update(data.world);
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
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, NiceGameData<'a, 'b>>,
        event: GameEvent,
    ) -> Trans<NiceGameData<'a, 'b>, GameEvent> {
        match event {
            GameEvent::Window(win_event) => {
                if is_key_down(&win_event, VirtualKeyCode::Escape) {
                    Trans::Push(Box::new(PauseState))
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }
}

struct PauseState;
impl<'a, 'b> State<NiceGameData<'a, 'b>, GameEvent> for PauseState {
    fn on_start(&mut self, data: StateData<'_, NiceGameData<'a, 'b>>) {
        data.world.read_resource::<AudioSink>().pause();
    }

    fn update(
        &mut self,
        data: StateData<'_, NiceGameData<'a, 'b>>,
    ) -> Trans<NiceGameData<'a, 'b>, GameEvent> {
        data.data.update(data.world);
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, NiceGameData<'a, 'b>>,
        event: GameEvent,
    ) -> Trans<NiceGameData<'a, 'b>, GameEvent> {
        match event {
            GameEvent::Window(win_event) => {
                if is_key_down(&win_event, VirtualKeyCode::Escape) {
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
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

fn transiton_game_state<'a, 'b>(
    world: &mut World,
    num_players: u8,
) -> Trans<NiceGameData<'a, 'b>, GameEvent> {
    hide_ui(world, "main_menu_root");
    Trans::Replace(Box::new(GameplayState {
        num_players: num_players,
        ..GameplayState::default()
    }))
}

fn hide_ui(world: &mut World, name: &str) {
    let ui_entity = world.exec(|ui_finder: UiFinder<'_>| ui_finder.find(name));
    if let Some(ent) = ui_entity {
        let _ = world
            .write_storage::<HiddenPropagate>()
            .insert(ent, HiddenPropagate::new());
    }
}
