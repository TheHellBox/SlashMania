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

use specs::World;
use clap::{Arg, App};

fn main() {
    println!("Starting SlashMania");
    let matches = App::new("Slash Mania")
        .arg(Arg::with_name("song")
            .short("s")
            .value_name("SONG")
            .takes_value(true))
        .arg(Arg::with_name("difficulty")
            .short("d")
            .value_name("DIFFICULTY")
            .takes_value(true))
        .get_matches();

    let song_name = matches.value_of("song").unwrap_or("Test Song").to_string();
    let difficulty = matches.value_of("difficulty").unwrap_or("Expert").to_string();

    let mut world = World::new();
    components::register_default(&mut world);

    let mut window = render::Window::new();
    window.compile_shaders();
    window.load_default_models();
    window.load_default_textures();

    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(components::sound::SoundSystem::new(), "Sound System", &[])
        .with(components::note::NoteSystem{..Default::default()}, "Note System", &[])
        .with_thread_local(window)
        .build();

    songs::load_song(song_name, difficulty, &mut world);
    'main: loop {
        dispatcher.dispatch(&mut world.res);
        let ents_to_remove = {
            let ents_to_remove_raw = &mut world.write_resource::<components::RemoveEntities>().0;
            let ents_to_remove = ents_to_remove_raw.clone();
            ents_to_remove_raw.clear();
            ents_to_remove
        };
        world.delete_entities(&ents_to_remove).unwrap();
    }
}
