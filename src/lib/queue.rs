// basically this is a rip off of the queues crate
#[derive(Debug)]
pub struct Queue<T: Clone> {
    queue: Vec<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T>{
        Queue{queue: vec![]}
    }

    pub fn add(&mut self, element: T) {
        self.queue.push(element);
    }

    pub fn remove(&mut self) -> Result<T, &str> {
        if !self.queue.is_empty() {
            Ok(self.queue.remove(0usize))
        } else {
            Err("queue is empty")
        }
    }

    pub fn peek(&self) -> Result<T, &str> {
        match self.queue.first() {
            Some(val) => Ok(val.clone()),
            None => Err("queue is empty"),
        }
    }
}
