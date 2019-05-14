use crate::components::*;
use crate::parser::*;
use specs::{Component, Join, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Note {
    pub line_layer: u8,
    pub line_index: u8,
    pub note_type: NoteType,
    pub time: f32,
    pub direction: Direction,
}

#[derive(Default)]
pub struct NoteSystem {
    pub last_update_time_ms: u128,
}

impl<'a> specs::System<'a> for NoteSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::Write<'a, RemoveEntities>,
        specs::Write<'a, sound::SoundEvents>,
        specs::WriteStorage<'a, drawable::Drawable>,
        specs::WriteStorage<'a, transform::Transform>,
        specs::ReadStorage<'a, Note>,
    );

    fn run(
        &mut self,
        (ents, mut ents_to_remove, mut sounds, mut drawables, mut transforms, notes): Self::SystemData,
    ) {
        let sys_time = std::time::SystemTime::now();
        let current_time = sys_time
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        if self.last_update_time_ms != 0 {
            let time_diff = (current_time - self.last_update_time_ms) as f32;
            for (ent, transform, note, drawable) in (&ents, &mut transforms, &notes, &mut drawables).join() {
                transform.position.vector =
                    transform.position.vector - nalgebra::Vector3::new(0.0, 0.0, time_diff / 60.0);
                let (remove_position, sound_enabled) = match note.note_type {
                    crate::parser::NoteType::Mine => (-5.0, false),
                    _ => (5.0, true)
                };
                if transform.position.z < remove_position {
                    ents_to_remove.0.push(ent);
                    if sound_enabled{
                        sounds.queue.push(sound::SoundEvent::AddSound("./assets/sounds/slash.mp3".to_string(), None));
                    }
                }
                if transform.position.z > 40.0{
                    drawable.enabled = false;
                }
                else{
                    drawable.enabled = true;
                }
            }
        }
        self.last_update_time_ms = current_time;
    }
}
