use amethyst::{
    core::{
        shrev::{EventChannel, ReaderId},
        EventReader,
    },
    derive::EventReader,
    ecs::{Read, SystemData, World},
    input::{BindingTypes, InputEvent, StringBindings},
    ui::UiEvent,
    winit::Event,
};

#[derive(Clone, Debug)]
pub struct ScoreEvent {
    // side: crate::components::paddle::Side,
    data: i32,
}

// #[derive(EventReader, Clone, Debug)]
// #[reader(GameEventReader)]
// pub enum GameEvent <T = StringBindings> where T: BindingTypes + Clone, {
//     Window(Event),
//     Ui(UiEvent),
//     Input(InputEvent<T>),
//     Score(ScoreEvent),
// }

#[derive(EventReader, Clone, Debug)]
#[reader(GameEventReader)]
pub enum GameEvent {
    Window(Event),
    Ui(UiEvent),
    Score(ScoreEvent),
}