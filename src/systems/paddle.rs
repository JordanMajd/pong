use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::paddle::{Paddle, Side, PADDLE_HEIGHT, PADDLE_VELOCITY};
use crate::pong::ARENA_HEIGHT;

#[derive(SystemDesc)]
pub struct PaddleSystem;
pub const PADDLE_SYSTEM: &str = "paddle_system";
impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut paddles, input, time): Self::SystemData) {
        for (paddle, transform) in (&mut paddles, &mut transforms).join() {
            let input_val = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(input_amount) = input_val {
                if input_amount != 0.0 {
                    let translate_amount =
                        time.delta_seconds() * PADDLE_VELOCITY * input_amount as f32;
                    let paddle_y = transform.translation().y;
                    if (paddle_y + translate_amount < ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                        && (paddle_y + translate_amount >= PADDLE_HEIGHT * 0.5)
                    {
                        paddle.velocity = translate_amount;
                    } else {
                        paddle.velocity = 0.0;
                    }
                } else {
                    paddle.velocity = 0.0;
                }
            }
        }
    }
}

pub struct PaddleMoveSystem;
pub const PADDLE_MOVE_SYSTEM: &str = "paddle_move_system";
impl<'s> System<'s> for PaddleMoveSystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Paddle>);
    fn run(&mut self, (mut transforms, paddles): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            transform.prepend_translation_y(paddle.velocity);
        }
    }
}
