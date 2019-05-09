pub mod drawable;
pub mod note;
pub mod obstacle;
pub mod transform;

pub fn register_default(world: &mut specs::World) {
    world.register::<note::Note>();
    world.register::<obstacle::Obstacle>();
    world.register::<transform::Transform>();
    world.register::<drawable::Drawable>();

    world.add_resource(CurrentSongInfo{..Default::default()});
}

#[derive(Default)]
pub struct CurrentSongInfo {
    pub bpm: i32,
    pub bpb: i32,
    pub time: i32
}
