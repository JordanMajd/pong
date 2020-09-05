use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, System, SystemData, WriteStorage},
};

use crate::audio::{play_score_sound, Sounds};
use crate::components::ball::{Ball, BALL_VELOCITY_X, BALL_VELOCITY_Y};
// use crate::components::scoreboard::{ScoreBoard};
use crate::pong::{ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct ScoreSystem;
pub const SCORE_SYSTEM: &str = "score_system";
impl<'s> System<'s> for ScoreSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (mut balls, mut locals, storage, sounds, audio_output): Self::SystemData,
    ) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let bx = transform.translation().x;
            let did_hit = if bx <= ball.radius {
                // TODO triggre score event
                true
            } else if bx >= ARENA_WIDTH - ball.radius {
                // TODO trigger score event
                true
            } else {
                false
            };

            if did_hit {
                play_score_sound(&*sounds, &storage, audio_output.as_deref());

                ball.velocity[0] = BALL_VELOCITY_X * ball.velocity[0].min(-1.).max(1.);
                ball.velocity[1] = BALL_VELOCITY_Y * ball.velocity[1].min(-1.).max(1.);

                transform.set_translation_x(ARENA_WIDTH / 2.0);
                transform.set_translation_y(ARENA_HEIGHT / 2.0);
            }
        }
    }
}
