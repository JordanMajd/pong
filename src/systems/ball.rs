use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage, Entity},
};

use crate::components::ball::{Ball, Dead};

#[derive(SystemDesc)]
pub struct BallMoveSystem;
pub const BALL_MOVE_SYSTEM: &str = "ball_move_system";
impl<'s> System<'s> for BallMoveSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Dead>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut locals, deads, time): Self::SystemData) {
        for (ball, local, ()) in (&balls, &mut locals, !&deads).join() {
            local.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
        }
    }
}
