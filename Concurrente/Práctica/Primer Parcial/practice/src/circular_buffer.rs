use std::sync::{Condvar, Mutex};

struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T> CircularBuffer<T> {
    pub fn add(&mut self, element: T) -> bool {
        if self.size == self.capacity {
            return false;
        }
        let i = self.head;
        self.buffer[i] = Some(element);
        self.head = (i + 1) % self.capacity;
        self.size += 1;
        true
    }
    pub fn remove(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let i = self.tail;
        let result = self.buffer[i].take();
        self.tail = (i + 1) % self.capacity;
        self.size -= 1;
        result
    }
}

struct Data<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    tail: usize,
    size: usize,
}
pub struct ConcurrentCircularBuffer<T> {
    data: Mutex<Data<T>>,
    not_empty: Condvar,
    not_full: Condvar,
}

impl<T> ConcurrentCircularBuffer<T> {
    pub fn add(&mut self, element: T) {
        let mut data = self.data.lock().unwrap();
        while data.size == data.capacity {
            data = self.not_full.wait(data).unwrap();
        }

        data.buffer[data.head] = Some(element);
        data.head = (data.head + 1) % data.capacity;
        data.size += 1;

        self.not_empty.notify_one();
    }
    pub fn remove(&mut self) -> T {
        let mut data = self.data.lock().unwrap();
        while data.size == 0 {
            data = self.not_empty.wait(data).unwrap();
        }

        let result = data.buffer[data.tail].take();
        data.tail = (data.tail + 1) % data.capacity;
        data.size -= 1;

        self.not_empty.notify_one();

        result.unwrap()
    }
}
