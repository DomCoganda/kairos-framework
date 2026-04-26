use std::sync::Arc;
use std::sync::Mutex;
use std::collections::VecDeque;

pub struct SignalBuffer<T > {
    inner: Arc<Mutex<VecDeque<T>>>,
    capacity: usize
}

impl<T> SignalBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        SignalBuffer {
            inner: Arc::new(Mutex::new(VecDeque::new())),
            capacity,
        }
    }
    ///
    pub fn push(&self, value: T) {
        let mut buffer = self.inner.lock().unwrap();
        if buffer.len() == self.capacity {
            buffer.pop_front();
        }
        buffer.push_back(value);
    }
    ///
    pub fn read(&self) -> Vec<T> where T: Clone {
        let buffer = self.inner.lock().unwrap();
        buffer.iter().cloned().collect()
    }
}
impl<T> Clone for SignalBuffer<T> {
    fn clone(&self) -> Self {
        SignalBuffer {
            inner: Arc::clone(&self.inner),
            capacity: self.capacity,
        }
    }
}