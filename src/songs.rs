use specs::Builder;

pub fn load_song_from_file(file: &std::path::Path, world: &mut specs::World) {
    let (notes, obstacle) = crate::parser::open_file(file);
    for note in notes {
        let position = crate::components::transform::Transform::new(
            nalgebra::Translation3::new(note.line_index as f32, note.line_layer as f32, note.time * 5.0),
            nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            nalgebra::Vector3::new(0.3, 0.3, 0.3),
        );
        let note_texture = match note.note_type {
            0 => "note_red",
            1 => "note_blue",
            2 => "mine",
            _ => "dev",
        }
        .to_string();
        let drawable = crate::components::drawable::Drawable::new(
            "block".to_string(),
            note_texture,
            "simple".to_string(),
        );
        world
            .create_entity()
            .with(position)
            .with(note)
            .with(drawable)
            .build();
    }
}

pub fn load_song(name: String, difficulty: String, world: &mut specs::World) {
    // Yes yes, that's not real song name, that's just an folder name. I know it. I'm just too lazy to do something better FIXME
    load_song_from_file(
        &std::path::Path::new(&format!("./assets/songs/{}/{}.json", name, difficulty)),
        world,
    )
}
