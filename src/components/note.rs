use crate::components::*;
use specs::{Component, Join, VecStorage};

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Note {
    pub line_layer: u8,
    pub line_index: u8,
    pub note_type: u8,
    pub time: f32,
    pub direction: u8,
}

#[derive(Default)]
pub struct NoteSystem {
    pub last_update_time_ms: u128,
}

impl<'a> specs::System<'a> for NoteSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::WriteStorage<'a, drawable::Drawable>,
        specs::WriteStorage<'a, transform::Transform>,
        specs::ReadStorage<'a, Note>,
        specs::Write<'a, RemoveEntities>,
    );

    fn run(
        &mut self,
        (ents, mut drawables, mut transforms, notes, mut ents_to_remove): Self::SystemData,
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
                    transform.position.vector - nalgebra::Vector3::new(0.0, 0.0, time_diff / 100.0);
                if transform.position.z < 5.0 {
                    ents_to_remove.0.push(ent);
                }
                if transform.position.z > 15.0{
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
