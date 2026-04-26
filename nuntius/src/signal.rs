use std::sync::Arc;
use std::sync::Mutex;

pub struct Signal<T> {
    inner: Arc<Mutex<T>>,
}

impl <T> Signal<T> {
    pub fn new(value: T) -> Self {
        Signal {
            inner: Arc::new(Mutex::new(value)),
        }
    }
    pub fn get(&self) -> T where T: Copy {
        *self.inner.lock().unwrap()
    }

    pub fn get_clone(&self) -> T where T: Clone {
        self.inner.lock().unwrap().clone()
    }

    pub fn set(&self, value: T) {
        *self.inner.lock().unwrap()= value
    }

}
impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Signal {
            inner: Arc::clone(&self.inner),
        }
    }
}