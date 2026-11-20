use std::collections::VecDeque;
use crate::music::Music;

#[derive(Default)]
enum RepeatMode{
    #[default]
    None,

    QueueRepeat,
    SingleRepeat,
}

#[derive(Default)]
struct Queue {
    queue: VecDeque<Music>,
    current: i16,
}

impl Queue {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, data: Music) {
        self.queue.insert(0, data);
    }
}