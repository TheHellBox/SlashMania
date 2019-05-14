use crate::parser::*;
use crate::components::*;
use nalgebra::UnitQuaternion;
use specs::Builder;

pub fn place_note(world: &mut specs::World, note: note::Note){
    let mut note_texture = match note.note_type {
        NoteType::Red => "note_red",
        NoteType::Blue => "note_blue",
        NoteType::Mine => "mine",
    }
    .to_string();

    let note_model = match note.note_type {
        NoteType::Mine => "mine",
        _ => "block"
    }
    .to_string();

    let note_direction = match note.direction {
        Direction::Bottom => UnitQuaternion::from_euler_angles(0.0, 0.0, 3.14),
        Direction::Top => UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),

        Direction::Right => UnitQuaternion::from_euler_angles(0.0, 0.0, 3.14 / 2.0),
        Direction::Left => UnitQuaternion::from_euler_angles(0.0, 0.0, -3.14 / 2.0),

        Direction::BottomRight => UnitQuaternion::from_euler_angles(0.0, 0.0, 3.14 / 4.0 * 3.0),
        Direction::BottomLeft => UnitQuaternion::from_euler_angles(0.0, 0.0, -3.14 / 4.0 * 3.0),

        Direction::TopRight => UnitQuaternion::from_euler_angles(0.0, 0.0, 3.14 / 4.0),
        Direction::TopLeft => UnitQuaternion::from_euler_angles(0.0, 0.0, -3.14 / 4.0),

        Direction::NoDirection => {
            note_texture = match note.note_type {
                NoteType::Red => "note_red_middle",
                NoteType::Blue => "note_blue_middle",
                NoteType::Mine => "mine",
            }
            .to_string();
            UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0)
        }
    };

    let drawable = drawable::Drawable::new(
        note_model,
        note_texture,
        "simple".to_string(),
    );
    let transform = transform::Transform::new(
        nalgebra::Translation3::new(
            -(note.line_index as f32) + 1.5,
            note.line_layer as f32 * 0.6 + 1.0,
            note.time / 60.0 + 5.0,
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

pub fn init_song(parsed_song: crate::parser::ParsedSong, world: &mut specs::World){
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
        place_note(world, note);
    }
}

pub fn load_song(name: String, difficulty: String, world: &mut specs::World) -> Result<(), std::io::Error>{
    let parsed_song = crate::parser::open_file(&std::path::Path::new(&format!("./assets/songs/{}/{}.json", name, difficulty)))?;
    let song_file = parsed_song.song_file.clone();
    // Yes yes, that's not real song name, that's just an folder name. I know it. I'm just too lazy to do something better FIXME
    init_song(
        parsed_song,
        world,
    );

    let mut sound_events = world.write_resource::<sound::SoundEvents>();
    let audio_start_event = sound::SoundEvent::AddSound(format!("./assets/songs/{}/{}", name, song_file), Some("SongPlayback".to_string()));
    sound_events.queue.push(audio_start_event);
    Ok(())
}
