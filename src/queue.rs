use std::collections::VecDeque;

#[derive(Default)]
struct Queue {
    queue: VecDeque<String>,
    current: i16,
}

impl Queue {
    pub fn new() -> Self {
        Self::default()
    }
}