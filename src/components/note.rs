use specs::{Component, VecStorage};

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Note{
    pub line_layer: i32,
    pub line_index: i32,
    pub note_type: i32,
    pub time: f32,
    pub direction: i32
}
