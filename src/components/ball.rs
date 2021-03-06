use crate::pong::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

pub const BALL_VELOCITY_X: f32 = 75.0 * 2.;
pub const BALL_VELOCITY_Y: f32 = 50.0 * 2.;
pub const BALL_RADIUS: f32 = 2.0;

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}
impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub struct Dead {
    pub time: f32,
}
impl Component for Dead {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    let ball = world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(transform)
        .build();
    ball
}
