#![allow(unused)]
use std::time::Instant;

pub enum Direction {
    Top = 0,
    Bottom = 1,
    Left = 2,
    Right = 3,

    TopLeft = 4,
    TopRight = 5,

    BottomLeft = 6,
    BottomRight = 7,
}

pub enum NoteType {
    Red = 0,
    Blue = 1,
    Mine = 3,
}

use std::fs::File;
use std::io::BufReader;
use crate::components::{note::Note, obstacle::Obstacle};

pub fn open_file(path: String)  {
    let start = Instant::now();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let json_content: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let bpm = json_content["_beatsPerMinute"].as_i64().expect("Cannot parse BPM") as i32;
    let bpb = json_content["_beatsPerBar"].as_i64().expect("Cannot parse BPB") as i32;
    let mut notes = vec![];
    let mut obstacles = vec![];
    if let serde_json::Value::Array(json_notes) = &json_content["_notes"]{
        for note in json_notes {
            let line_layer = note["_lineLayer"].as_i64().expect("Cannot parse note line layer") as i32;
            let line_index = note["_lineIndex"].as_i64().expect("Cannot parse note line index") as i32;
            let note_type  = note["_type"].as_i64().expect("Cannot parse note line type") as i32;
            let time       = note["_time"].as_f64().expect("Cannot parse note time") as f32;
            let direction  = note["_cutDirection"].as_i64().expect("Cannot parse note direction") as i32;

            notes.push(Note{
                line_layer,
                line_index,
                note_type,
                time,
                direction
            });
        }
    }

    if let serde_json::Value::Array(json_notes) = &json_content["_obstacles"]{
        for note in json_notes {
            let line_index     = note["_lineIndex"].as_i64().expect("Cannot parse note obstacle index") as i32;
            let obstacle_type  = note["_type"].as_i64().expect("Cannot parse obstacle line type") as i32;
            let time           = note["_time"].as_f64().expect("Cannot parse obstacle time") as f32;
            let duration       = note["_duration"].as_f64().expect("Cannot parse obstacle duration") as f32;
            let width          = note["_width"].as_i64().expect("Cannot parse obstacle width") as i32;

            obstacles.push(Obstacle{
                line_index,
                obstacle_type,
                time,
                duration,
                width
            });
        }
    }
    println!("Parsing took {} milliseconds", start.elapsed().as_millis());
}
