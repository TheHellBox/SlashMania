use crate::components::*;
use specs::{Component, Join, VecStorage};

pub enum ObstacleType {
    Wall = 0,
    Ceiling = 1
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Obstacle {
    pub line_index: i32,
    pub obstacle_type: ObstacleType,
    pub width: i32,
    pub time: f32,
    pub duration: f32,
}

#[derive(Default)]
pub struct ObstacleSystem {
    pub last_update_time_ms: u128,
}

impl<'a> specs::System<'a> for ObstacleSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::Write<'a, RemoveEntities>,
        specs::WriteStorage<'a, transform::Transform>,
        specs::ReadStorage<'a, Obstacle>,
    );

    fn run(
        &mut self,
        (ents, mut ents_to_remove, mut transforms, obstacles): Self::SystemData,
    ) {
        let sys_time = std::time::SystemTime::now();
        let current_time = sys_time
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        if self.last_update_time_ms != 0 {
            let time_diff = (current_time - self.last_update_time_ms) as f32;
            for (ent, transform, obstacle) in
                (&ents, &mut transforms, &obstacles).join()
            {
                transform.position.vector =
                    transform.position.vector - nalgebra::Vector3::new(0.0, 0.0, time_diff / 60.0);
                if transform.position.z < -obstacle.duration {
                    ents_to_remove.0.push(ent);
                }
            }
        }
        self.last_update_time_ms = current_time;
    }
}
