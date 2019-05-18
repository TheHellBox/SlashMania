use std::time::Instant;

pub struct ParsedSong {
    pub notes: Vec<Note>,
    pub obstacles: Vec<Obstacle>,
    pub bpm: f32,
    pub bpb: f32,
    pub time: i32,
    pub song_file: String,
}

use crate::components::{note::*, obstacle::*};
use std::fs::File;
use std::io::BufReader;

pub fn open_file(path: &std::path::Path) -> Result<ParsedSong, std::io::Error> {
    let start = Instant::now();

    let level_file = File::open(path)?;
    let level_reader = BufReader::new(level_file);

    let path_parent = path.parent().expect("Cannot find path parent");
    let info_file = File::open(path_parent.join("info.json"))?;
    let info_reader = BufReader::new(info_file);

    let level_json: serde_json::Value = serde_json::from_reader(level_reader)?;
    let info_json: serde_json::Value = serde_json::from_reader(info_reader)?;

    let bpm = level_json["_beatsPerMinute"]
        .as_f64()
        .expect("Cannot parse BPM") as f32;
    let bpb = level_json["_beatsPerBar"]
        .as_f64()
        .expect("Cannot parse BPB") as f32;
    let time = level_json["_time"].as_i64().unwrap_or(0) as i32;
    let bpms = 1000.0 * 60.0 / bpm; // beats per ms
                                    // FIXME: It will use song file defined for default difficulty
    let song_file = info_json["difficultyLevels"].as_array().unwrap()[0]["audioPath"]
        .as_str()
        .expect("Cannot parse audioPath")
        .to_string();

    let mut notes = vec![];
    let mut obstacles = vec![];
    if let serde_json::Value::Array(json_notes) = &level_json["_notes"] {
        for note in json_notes {
            let line_layer = note["_lineLayer"]
                .as_i64()
                .expect("Cannot parse note line layer") as u8;
            let line_index = note["_lineIndex"]
                .as_i64()
                .expect("Cannot parse note line index") as u8;
            let note_type = note["_type"].as_i64().expect("Cannot parse note line type") as u8;
            let time = note["_time"].as_f64().expect("Cannot parse note time") as f32 * bpms; // Time in ms
            let direction = note["_cutDirection"]
                .as_i64()
                .expect("Cannot parse note direction") as u8;

            let note_type: NoteType = match note_type {
                0 => NoteType::Red,
                1 => NoteType::Blue,
                _ => NoteType::Mine,
            };
            let direction: Direction = match direction {
                0 => Direction::Top,
                1 => Direction::Bottom,
                2 => Direction::Left,
                3 => Direction::Right,

                4 => Direction::TopLeft,
                5 => Direction::TopRight,

                6 => Direction::BottomLeft,
                7 => Direction::BottomRight,

                _ => Direction::NoDirection,
            };

            notes.push(Note {
                line_layer,
                line_index,
                note_type,
                time,
                direction,
            });
        }
    }

    if let serde_json::Value::Array(json_notes) = &level_json["_obstacles"] {
        for note in json_notes {
            let line_index = note["_lineIndex"]
                .as_i64()
                .expect("Cannot parse note obstacle index") as i32;
            let obstacle_type = note["_type"]
                .as_i64()
                .expect("Cannot parse obstacle line type") as i32;
            let time = note["_time"].as_f64().expect("Cannot parse obstacle time") as f32 * bpms;
            let duration = note["_duration"]
                .as_f64()
                .expect("Cannot parse obstacle duration") as f32
                * bpms;
            let width = note["_width"]
                .as_i64()
                .expect("Cannot parse obstacle width") as i32;

            let obstacle_type: ObstacleType = match obstacle_type {
                0 => ObstacleType::Wall,
                1 => ObstacleType::Ceiling,
                _ => ObstacleType::Wall,
            };

            obstacles.push(Obstacle {
                line_index,
                obstacle_type,
                time,
                duration,
                width,
            });
        }
    }
    println!("Parsing took {} milliseconds", start.elapsed().as_millis());
    Ok(ParsedSong {
        notes,
        obstacles,
        bpm,
        bpb,
        time,
        song_file,
    })
}
