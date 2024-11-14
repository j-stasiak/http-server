use std::collections::VecDeque;

pub struct ThreadPool {
    max_threads: u8,
    tasks_queue: VecDeque<_>,
}

impl ThreadPool {
    pub fn new(max_threads: u8) -> ThreadPool {
        ThreadPool { max_threads }
    }

    pub fn enque<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
