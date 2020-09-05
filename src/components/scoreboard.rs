use amethyst::ecs::{Component, DenseVecStorage};

pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

impl Component for ScoreBoard {
    type Storage = DenseVecStorage<Self>;
}

impl ScoreBoard {
    pub fn new() -> ScoreBoard {
        ScoreBoard {
            score_left: 0,
            score_right: 0,
        }
    }
}
