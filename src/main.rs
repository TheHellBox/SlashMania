#[macro_use]
extern crate glium;
#[macro_use]
extern crate specs_derive;

extern crate x11;
extern crate tobj;
extern crate specs;
extern crate serde;
extern crate openxr;
extern crate serde_json;

mod parser;
mod render;
mod textures;
mod components;
mod obj_loader;
mod openxr_module;

pub static SCALE: f32 = 5.0;

fn main() {
    let mut window = render::Window::new();

    'main: loop {
        let frame = window.draw();
        window.update_xr();
    }
}
