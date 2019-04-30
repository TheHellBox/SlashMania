use specs::{Component, VecStorage};

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Obstacle {
    pub line_index: i32,
    pub obstacle_type: i32,
    pub width: i32,
    pub time: f32,
    pub duration: f32,
}
