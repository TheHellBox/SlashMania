use crate::parser::*;
use crate::components::*;
use nalgebra::UnitQuaternion;
use specs::Builder;

pub fn load_song_from_file(file: &std::path::Path, world: &mut specs::World) {
    let parsed_song = crate::parser::open_file(file);
    {
        let parsed_song_info = CurrentSongInfo{
            bpm: parsed_song.bpm,
            bpb: parsed_song.bpb,
            time: parsed_song.time,
        };
        let mut song_info = world.write_resource::<CurrentSongInfo>();
        *song_info = parsed_song_info;
    }
    for note in parsed_song.notes {
        // FIXME: Unsafe is not good, but there is no other simple workarounds... Well there is but they require more code
        let note_type: NoteType = unsafe { std::mem::transmute(note.note_type) };
        let direction: Direction = unsafe { std::mem::transmute(note.direction) };

        let mut note_texture = match note_type {
            NoteType::Red => "note_red",
            NoteType::Blue => "note_blue",
            NoteType::Mine => "mine",
        }
        .to_string();

        let note_direction = match direction {
            Direction::Top => UnitQuaternion::from_euler_angles(0.0, 0.0, 3.14),
            Direction::Bottom => UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),

            Direction::Left => UnitQuaternion::from_euler_angles(0.0, 0.0, 3.14 / 2.0),
            Direction::Right => UnitQuaternion::from_euler_angles(0.0, 0.0, -3.14 / 2.0),

            Direction::TopLeft => UnitQuaternion::from_euler_angles(0.0, 0.0, 3.14 / 4.0 * 3.0),
            Direction::TopRight => UnitQuaternion::from_euler_angles(0.0, 0.0, -3.14 / 4.0 * 3.0),

            Direction::BottomLeft => UnitQuaternion::from_euler_angles(0.0, 0.0, 3.14 / 4.0),
            Direction::BottomRight => UnitQuaternion::from_euler_angles(0.0, 0.0, -3.14 / 4.0),

            Direction::NoDirection => {
                note_texture = match note_type {
                    NoteType::Red => "note_red_middle",
                    NoteType::Blue => "note_blue_middle",
                    NoteType::Mine => "mine",
                }
                .to_string();
                UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0)
            }
        };

        let drawable = drawable::Drawable::new(
            "block".to_string(),
            note_texture,
            "simple".to_string(),
        );

        let transform = transform::Transform::new(
            nalgebra::Translation3::new(
                -note.line_index as f32 + 1.5,
                note.line_layer as f32,
                note.time * 20.0,
            ),
            note_direction,
            nalgebra::Vector3::new(0.3, 0.3, 0.3),
        );

        world
            .create_entity()
            .with(transform)
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
