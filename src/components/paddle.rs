use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use super::ai::AI;
use crate::pong::{ARENA_HEIGHT, ARENA_WIDTH};

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_VELOCITY: f32 = 180.0;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
    pub velocity: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            velocity: 0.0,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, num_players: u8) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    if num_players == 0 {
        get_ai_paddle(world, sprite_render.clone(), left_transform, Side::Left)
    } else {
        get_paddle(world, sprite_render.clone(), left_transform, Side::Left)
    };

    if num_players <= 1 {
        get_ai_paddle(world, sprite_render, right_transform, Side::Right)
    } else {
        get_paddle(world, sprite_render, right_transform, Side::Right)
    };
}

fn get_ai_paddle(
    world: &mut World,
    sprite_render: SpriteRender,
    transform: Transform,
    side: Side,
) -> Entity {
    world
        .create_entity()
        .with(sprite_render)
        .with(Paddle::new(side))
        .with(transform)
        .with(AI)
        .build()
}

fn get_paddle(
    world: &mut World,
    sprite_render: SpriteRender,
    transform: Transform,
    side: Side,
) -> Entity {
    world
        .create_entity()
        .with(sprite_render)
        .with(Paddle::new(side))
        .with(transform)
        .build()
}
