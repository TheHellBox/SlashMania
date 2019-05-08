use specs::{Component, VecStorage};
#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub model: String,
    pub texture: String,
    pub shader: String,
}

impl Drawable {
    pub fn new(model: String, texture: String, shader: String) -> Self {
        Self {
            model,
            texture,
            shader,
        }
    }
}
