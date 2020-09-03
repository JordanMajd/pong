use crate::pong::{Ball, AI, PADDLE_VELOCITY};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct AIBigBrainSystem;
impl<'s> System<'s> for AIBigBrainSystem {
    type SystemData = (
        WriteStorage<'s, AI>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
    );
    fn run(&mut self, (mut ais, balls, transforms): Self::SystemData) {
        for (ai, at) in (&mut ais, &transforms).join() {
            let ay = at.translation().y;
            for (_, bt) in (&balls, &transforms).join() {
                let by = bt.translation().y;
                let mut vel = PADDLE_VELOCITY;
                if ay >= by {
                    vel = -vel;
                }
                ai.velocity = vel;
            }
        }
    }
}

pub struct AIMoveSystem;
impl<'s> System<'s> for AIMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, AI>,
        Read<'s, Time>,
    );
    fn run(&mut self, (mut transforms, ais, time): Self::SystemData) {
        for (ai, transform) in (&ais, &mut transforms).join() {
            transform.prepend_translation_y(ai.velocity * time.delta_seconds());
        }
    }
}
