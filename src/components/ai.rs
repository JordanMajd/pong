use amethyst::ecs::{Component, DenseVecStorage};

pub struct AI {
    pub velocity: f32,
}
impl Component for AI {
    type Storage = DenseVecStorage<Self>;
}
