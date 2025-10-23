use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let stack = Stack::new();

    {
        let stack = stack.clone();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        println!("Stack length: {}", stack.size());
        println!("Stack is empty: {}", stack.is_empty());

        if let Some(top) = stack.top() {
            println!("Top element: {}", top);
        }

        if let Some(pop) = stack.pop() {
            println!("Popped element: {}", pop);
        }

        println!("Stack is empty after popping: {}", stack.is_empty());

        stack.clear();
        println!("Stack is empty after clear: {}", stack.is_empty());
    }

    println!("\n--- Multi-threaded test ---");
    let stack_clone1 = stack.clone();
    let stack_clone2 = stack.clone();

    let handle1 = thread::spawn(move || {
        for i in 1..=5 {
            stack_clone1.push(i);
            println!("Thread 1 pushed: {}", i);
        }
    });

    let handle2 = thread::spawn(move || {
        for i in 6..=10 {
            stack_clone2.push(i);
            println!("Thread 2 pushed: {}", i);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final stack length: {}", stack.size());
}

struct Stack<T> {
    data: Arc<Mutex<Vec<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn push(&self, item: T) {
        if let Ok(mut data) = self.data.lock() {
            data.push(item);
        }
    }

    fn pop(&self) -> Option<T> {
        if let Ok(mut data) = self.data.lock() {
            data.pop()
        } else {
            None
        }
    }

    fn top(&self) -> Option<T>
    where
        T: Clone,
    {
        if let Ok(data) = self.data.lock() {
            data.last().cloned()
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
}

impl<T> Clone for Stack<T> {
    fn clone(&self) -> Self {
        Stack {
            data: Arc::clone(&self.data),
        }
    }
}
