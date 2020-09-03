use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::audio::{play_bounce_sound, Sounds};
use crate::pong::{Ball, Paddle, Side, ARENA_HEIGHT};

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (mut balls, paddles, transforms, storage, sounds, audio_output): Self::SystemData,
    ) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let bx = transform.translation().x;
            let by = transform.translation().y;

            if (by <= ball.radius && ball.velocity[1] < 0.0)
                || (by >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0)
            {
                ball.velocity[1] = -ball.velocity[1];
                play_bounce_sound(&*sounds, &storage, audio_output.as_deref());
            }

            for (paddle, transform) in (&paddles, &transforms).join() {
                let px = transform.translation().x - (paddle.width * 0.5);
                let py = transform.translation().y - (paddle.height * 0.5);
                if point_in_rect(
                    bx,
                    by,
                    px - ball.radius,
                    py - ball.radius,
                    px + paddle.width + ball.radius,
                    py + paddle.height + ball.radius,
                ) {
                    if (paddle.side == Side::Left && ball.velocity[0] < 0.0)
                        || (paddle.side == Side::Right && ball.velocity[0] > 0.0)
                    {
                        ball.velocity[0] = -ball.velocity[0];
                        play_bounce_sound(&*sounds, &storage, audio_output.as_deref());
                    }
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
