use std::sync::{Arc, Mutex};

fn main() {
    let queue = Queue::new();
    
    println!("=== Test queue ===");
    println!("Empty queue: is_empty={}, size={}", queue.is_empty(), queue.size());
    
    println!("\nAdd elements: 1, 2, 3");
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    
    println!("Queue size: {}, is_empty: {}", queue.size(), queue.is_empty());
    
    if let Some(val) = queue.peek() {
        println!("First element (peek): {}", val);
    }
    
    println!("\nRemove elements:");
    for _ in 0..2 {
        if let Some(val) = queue.dequeue() {
            println!("Removed element: {}", val);
        } else {
            println!("Queue is empty!");
        }
    }
    queue.to_slice();
    println!("Queue elements: {:?}", queue.to_slice());

    println!("Queue size: {}, is_empty: {}", queue.size(), queue.is_empty());
    queue.clear();
    println!("Queue is empty after clear: {}", queue.is_empty());
    println!("Final size: {}", queue.size());
}

struct Queue<T> {
    data: Arc<Mutex<Vec<T>>>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn enqueue(&self, item: T) {
        self.data.lock().unwrap().push(item);
    }

    fn dequeue(&self) -> Option<T> {
        if let Ok(mut data) = self.data.lock() {
            if data.is_empty() {
                None
            } else {
                Some(data.remove(0))
            }
        } else {
            None
        }
    }

    fn peek(&self) -> Option<T> 
    where 
        T: Clone 
    {
        if let Ok(data) = self.data.lock() {
            if data.is_empty() {
                None
            } else {
                Some(data[0].clone())
            }
        } else {
            None
        }
    }
    
    fn is_empty(&self) -> bool {
        if let Ok(data) = self.data.lock() {
            data.is_empty()
        } else {
            true
        }
    }

    fn size(&self) -> usize {
        if let Ok(data) = self.data.lock() {
            data.len()
        } else {
            0
        }
    }
    
    fn clear(&self) {
        if let Ok(mut data) = self.data.lock() {
            data.clear();
        }
    }

    fn to_slice(&self) -> Vec<T> 
    where 
        T: Clone 
    {
        if let Ok(data) = self.data.lock() {
            data.clone()
        } else {
            Vec::new()
        }
    }
}