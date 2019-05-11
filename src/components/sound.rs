use std::collections::HashMap;

pub struct SoundSystem {
    device: rodio::Device,
    sounds: HashMap<String, rodio::Sink>,
}

pub enum SoundEvent {
    // Option<String> is a name. A name is beeing used if you want to pause or continue sound, leave None if you want to play it once
    AddSound(String, Option<String>),
    PauseSound(String),
    ContinueSound(String),
}

#[derive(Default)]
pub struct SoundEvents {
    pub queue: Vec<SoundEvent>,
}

impl<'a> specs::System<'a> for SoundSystem {
    type SystemData = (specs::Write<'a, SoundEvents>);

    fn run(&mut self, mut sound_events: Self::SystemData) {
        for event in &sound_events.queue {
            match event {
                SoundEvent::AddSound(path, name) => {
                    let file = std::fs::File::open(path).unwrap();
                    let sink = rodio::Sink::new(&self.device);
                    sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).unwrap());
                    if let Some(name) = name {
                        self.sounds.insert(name.clone(), sink);
                    } else {
                        sink.detach();
                    }
                },
                SoundEvent::PauseSound(name) => {
                    if let Some(sink) = self.sounds.get(name) {
                        sink.pause();
                    }
                },
                SoundEvent::ContinueSound(name) => {
                    if let Some(sink) = self.sounds.get(name) {
                        sink.play();
                    }
                },
            }
        }
        sound_events.queue.clear();
    }
}

impl SoundSystem {
    pub fn new() -> Self {
        let device = rodio::default_output_device().unwrap();
        Self {
            device,
            sounds: HashMap::with_capacity(64),
        }
    }
}
