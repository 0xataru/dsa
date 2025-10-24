# Queue Data Structure (Rust Implementation)

A queue is a linear data structure that follows the First In, First Out (FIFO) principle. Elements are added to the rear and removed from the front, like people waiting in line.

## Operations

- **Enqueue**: Add an element to the rear of the queue
- **Dequeue**: Remove and return the front element from the queue
- **Peek/Front**: View the front element without removing it
- **IsEmpty**: Check if the queue is empty
- **Size**: Get the number of elements in the queue
- **Clear**: Remove all elements from the queue
- **ToSlice**: Get a copy of all elements in order

## Implementation Details

This implementation uses:
- **Vec<T>**: Dynamic array as the underlying data structure
- **Arc<Mutex<Vec<T>>>**: Thread-safe wrapper for concurrent access
- **Generic type T**: Supports any data type
- **Clone bounds**: Required for peek and to_slice operations

## Time Complexity

- **Enqueue**: O(1) amortized
- **Dequeue**: O(n) - removes first element, shifts all others
- **Peek**: O(1)
- **IsEmpty**: O(1)
- **Size**: O(1)
- **Clear**: O(1)
- **ToSlice**: O(n) - clones all elements

## Space Complexity

- **Overall**: O(n) where n is the number of elements
- **Thread Safety**: Additional overhead for Arc<Mutex<>>

## Examples

**Basic Usage:**
```rust
let queue = Queue::new();
queue.enqueue(1);
queue.enqueue(2);
queue.enqueue(3);

println!("Size: {}", queue.size()); // Size: 3
println!("Front: {:?}", queue.peek()); // Front: Some(1)

if let Some(value) = queue.dequeue() {
    println!("Dequeued: {}", value); // Dequeued: 1
}
```

**Thread-Safe Operations:**
```rust
let queue = Queue::new();
let queue_clone = queue.clone();

// Safe to use across threads
thread::spawn(move || {
    queue_clone.enqueue(42);
});
```

**Complete Example:**
```rust
let queue = Queue::new();

// Add elements
queue.enqueue(1);
queue.enqueue(2);
queue.enqueue(3);

// Check state
println!("Size: {}, Empty: {}", queue.size(), queue.is_empty());

// Peek at front
if let Some(val) = queue.peek() {
    println!("Front element: {}", val);
}

// Remove elements
for _ in 0..2 {
    if let Some(val) = queue.dequeue() {
        println!("Removed: {}", val);
    }
}

// Get all remaining elements
println!("Remaining: {:?}", queue.to_slice());

// Clear queue
queue.clear();
println!("After clear - Empty: {}", queue.is_empty());
```

## Key Features

1. **Thread Safety**: Uses Arc<Mutex<>> for concurrent access
2. **Memory Safety**: Rust's ownership system prevents memory leaks
3. **Generic**: Works with any data type
4. **Clone Support**: Can be shared between threads
5. **Error Handling**: Graceful handling of mutex lock failures
6. **FIFO Semantics**: Proper queue behavior with first-in-first-out

## Use Cases

- Process scheduling (CPU, I/O)
- Breadth-First Search (BFS) algorithms
- Print job management
- Web server request handling
- Message queues and buffers
- Task scheduling systems
- Event processing pipelines

## Rust-Specific Features

- **Ownership**: Automatic memory management
- **Borrowing**: Safe concurrent access with Arc<Mutex<>>
- **Generics**: Type-safe implementation
- **Error Handling**: Option<T> for safe operations
- **Clone Bounds**: Required for peek and to_slice
- **Thread Safety**: Safe to share between threads

## Performance Considerations

### Current Implementation
- **Enqueue**: O(1) - efficient push to end
- **Dequeue**: O(n) - inefficient remove from front
- **Memory**: Additional overhead for thread safety

### Optimization Opportunities
For better performance, consider:
- **VecDeque<T>**: Built-in double-ended queue with O(1) operations
- **Circular Buffer**: Custom implementation for O(1) all operations
- **Lock-free**: For high-performance scenarios

## Best Practices

1. **Clone Bounds**: Add `T: Clone` where needed
2. **Error Handling**: Always handle Option<T> returns
3. **Thread Safety**: Safe for concurrent access
4. **Memory Management**: Rust handles automatically
5. **Performance**: Consider VecDeque for high-frequency operations

## Comparison with Standard Library

### Current Implementation
```rust
// Custom queue with Vec<T>
let queue = Queue::new();
queue.enqueue(1);
let val = queue.dequeue(); // O(n)
```

### Standard Library Alternative
```rust
// VecDeque for better performance
use std::collections::VecDeque;
let mut queue = VecDeque::new();
queue.push_back(1);
let val = queue.pop_front(); // O(1)
```

## Thread Safety Details

The implementation uses `Arc<Mutex<Vec<T>>>` which provides:
- **Arc**: Atomic Reference Counting for shared ownership
- **Mutex**: Mutual exclusion for thread safety
- **Clone**: Safe sharing between threads
- **Lock Management**: Automatic unlocking with RAII

## Common Patterns

### BFS Implementation
```rust
fn bfs(graph: &Graph, start: Node) -> Vec<Node> {
    let mut queue = Queue::new();
    let mut visited = HashSet::new();
    
    queue.enqueue(start);
    visited.insert(start);
    
    while let Some(node) = queue.dequeue() {
        for neighbor in graph.neighbors(node) {
            if !visited.contains(&neighbor) {
                queue.enqueue(neighbor);
                visited.insert(neighbor);
            }
        }
    }
    visited.into_iter().collect()
}
```

### Level Order Traversal
```rust
fn level_order_traversal(root: Option<&TreeNode>) -> Vec<Vec<i32>> {
    let mut queue = Queue::new();
    let mut result = Vec::new();
    
    if let Some(node) = root {
        queue.enqueue(node);
    }
    
    while !queue.is_empty() {
        let level_size = queue.size();
        let mut level = Vec::new();
        
        for _ in 0..level_size {
            if let Some(node) = queue.dequeue() {
                level.push(node.val);
                if let Some(left) = &node.left {
                    queue.enqueue(left);
                }
                if let Some(right) = &node.right {
                    queue.enqueue(right);
                }
            }
        }
        result.push(level);
    }
    result
}
```
