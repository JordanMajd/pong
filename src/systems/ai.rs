use crate::components::{
    ai::AI,
    ball::Ball,
    paddle::{Paddle, PADDLE_VELOCITY},
};

use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

pub struct AIBigBrainSystem;
impl<'s> System<'s> for AIBigBrainSystem {
    type SystemData = (
        ReadStorage<'s, AI>,
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
    );
    fn run(&mut self, (ais, balls, mut paddles, transforms, time): Self::SystemData) {
        for (_, paddle, at) in (&ais, &mut paddles, &transforms).join() {
            let ay = at.translation().y;
            for (_, bt) in (&balls, &transforms).join() {
                let by = bt.translation().y;
                let mut vel = PADDLE_VELOCITY;
                if ay >= by {
                    vel = -vel;
                }
                paddle.velocity = vel * time.delta_seconds();
            }
        }
    }
}
