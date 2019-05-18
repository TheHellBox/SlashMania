use crate::components::note::*;
use crate::components::*;
use nalgebra::UnitQuaternion;
use specs::Builder;

pub fn place_note(world: &mut specs::World, note: note::Note) {
    let note_texture = match note.note_type {
        NoteType::Red => "note_red",
        NoteType::Blue => "note_blue",
        NoteType::Mine => "mine",
    }
    .to_string();

    let note_model = match note.note_type {
        NoteType::Mine => "mine",
        _ => "block",
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

        Direction::NoDirection => UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
    };

    let drawable = drawable::Drawable::new(note_model, note_texture, "simple".to_string());
    let transform = transform::Transform::new(
        nalgebra::Translation3::new(
            -(note.line_index as f32 * 0.7) + 1.0,
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

pub fn place_obstacle(world: &mut specs::World, obstacle: obstacle::Obstacle) {
    let scale = match obstacle.obstacle_type {
        obstacle::ObstacleType::Wall => {
            nalgebra::Vector3::new(obstacle.width as f32 * 0.3, 2.0, obstacle.duration / 60.0)
        }
        obstacle::ObstacleType::Ceiling => {
            nalgebra::Vector3::new(1.2, obstacle.width as f32 * 0.3, obstacle.duration / 60.0)
        }
    };
    let position = match obstacle.obstacle_type {
        obstacle::ObstacleType::Wall => {
            nalgebra::Translation3::new(
                -(obstacle.line_index as f32 * 1.5) + 2.0,
                2.0,
                obstacle.time / 60.0 + 5.0 + obstacle.duration / 120.0,
            )
        }
        obstacle::ObstacleType::Ceiling => {
            nalgebra::Translation3::new(
                -(obstacle.line_index as f32 * 1.5) + 0.8,
                3.2 - (obstacle.width / 2) as f32,
                obstacle.time / 60.0 + 5.0 + obstacle.duration / 120.0,
            )
        }
    };
    let transform = transform::Transform::new(
        position,
        UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
        scale,
    );
    let drawable = drawable::Drawable::new(
        "cube".to_string(),
        "obstacle".to_string(),
        "wall".to_string(),
    );
    world
        .create_entity()
        .with(transform)
        .with(obstacle)
        .with(drawable)
        .build();
}

pub fn init_song(parsed_song: crate::parser::ParsedSong, world: &mut specs::World) {
    {
        let parsed_song_info = CurrentSongInfo {
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
    for obstacle in parsed_song.obstacles {
        place_obstacle(world, obstacle);
    }
}

pub fn load_song(
    name: String,
    difficulty: String,
    world: &mut specs::World,
) -> Result<(), std::io::Error> {
    let parsed_song = crate::parser::open_file(&std::path::Path::new(&format!(
        "./assets/songs/{}/{}.json",
        name, difficulty
    )))?;
    let song_file = parsed_song.song_file.clone();
    // Yes yes, that's not real song name, that's just an folder name. I know it. I'm just too lazy to do something better FIXME
    init_song(parsed_song, world);

    let mut sound_events = world.write_resource::<sound::SoundEvents>();
    let audio_start_event = sound::SoundEvent::AddSound(
        format!("./assets/songs/{}/{}", name, song_file),
        Some("SongPlayback".to_string()),
    );
    sound_events.queue.push(audio_start_event);
    Ok(())
}
