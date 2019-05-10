pub mod drawable;
pub mod note;
pub mod obstacle;
pub mod sound;
pub mod transform;

pub fn register_default(world: &mut specs::World) {
    world.register::<note::Note>();
    world.register::<obstacle::Obstacle>();
    world.register::<transform::Transform>();
    world.register::<drawable::Drawable>();

    world.add_resource(CurrentSongInfo {
        ..Default::default()
    });
    world.add_resource(sound::SoundEvents {
        ..Default::default()
    });
    world.add_resource(RemoveEntities {
        ..Default::default()
    });
}

#[derive(Default)]
pub struct CurrentSongInfo {
    pub bpm: i32,
    pub bpb: i32,
    pub time: i32,
}

#[derive(Default)]
pub struct RemoveEntities(pub Vec<specs::Entity>);
