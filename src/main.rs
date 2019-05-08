#[macro_use]
extern crate glium;
#[macro_use]
extern crate specs_derive;

mod components;
mod obj_loader;
mod openxr_module;
mod parser;
mod render;
mod songs;
mod textures;

pub static SCALE: f32 = 5.0;

use specs::World;

fn main() {
    let mut world = World::new();
    components::register_default(&mut world);

    let mut window = render::Window::new();
    window.compile_shaders();
    window.load_default_models();
    window.load_default_textures();

    let mut dispatcher = specs::DispatcherBuilder::new()
        .with_thread_local(window)
        .build();

    songs::load_song("Test Song".to_string(), "Expert".to_string(), &mut world);
    'main: loop {
        dispatcher.dispatch(&mut world.res);
    }
}
