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
    current: usize,
}

impl Queue {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, index:usize, music: Music) {
        self.queue.insert(index, music)
    }

    pub fn add(&mut self, music: Music) {
        self.queue.push_back(music)
    }

    pub fn remove_from_index(&mut self, index:usize) -> bool {
        self.queue.remove(index).is_some()
    }
}