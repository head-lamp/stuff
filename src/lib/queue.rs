use std::collections::VecDeque;

#[derive(Debug)]
pub struct Queue<T: Clone> {
    queue: VecDeque<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new(size: usize) -> Queue<T>{
        Queue{queue: VecDeque::<T>::with_capacity(size)}
    }

    pub fn add(&mut self, element: T) {
        self.queue.push_back(element);
    }

    pub fn remove(&mut self) -> Result<T, &str> {
        if !self.queue.is_empty() {
            match self.queue.remove(0 as usize) {
                Some(x) => Ok(x),
                None => Err("invalid_index")
            }
        } else {
            Err("queue is empty")
        }
    }

    pub fn peek(&self) -> Result<T, &str> {
        match self.queue.get(0) {
            Some(val) => Ok(val.clone()),
            None => Err("queue is empty"),
        }
    }
}
