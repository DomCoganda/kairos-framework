use crate::buffer::SignalBuffer;
use crate::signal::Signal;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct Filus<Value> {
    pub current: Signal<Option<Value>>,
    pub history: SignalBuffer<Value>,
}

pub struct Broker<Key,Value> {
    hash_map: HashMap<Key, Filus<Value>>,
}

impl<Key: Hash + Eq, Value: Clone> Broker<Key, Value> {
    pub fn new() -> Broker<Key,Value> {
        Broker {
            hash_map: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, key: Key) -> Filus<Value> {
        self.hash_map.entry(key).or_insert_with(|| Filus {
            current: Signal::new(None),
            history: SignalBuffer::new(60),
        }).clone()
    }

    pub fn publish(&mut self, key: Key, value: Value) {
        if let Some(filus) = self.hash_map.get(&key) {
            filus.history.push(value.clone());
            filus.current.set(Some(value));
        }
    }
}