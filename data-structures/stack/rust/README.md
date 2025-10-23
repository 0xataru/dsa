# Stack Data Structure (Rust Implementation)

A stack is a linear data structure that follows the Last In, First Out (LIFO) principle. Elements are added and removed from the same end, called the "top" of the stack.

## Operations

- **Push**: Add an element to the top of the stack
- **Pop**: Remove and return the top element from the stack
- **Top/Peek**: View the top element without removing it
- **IsEmpty**: Check if the stack is empty
- **Size**: Get the number of elements in the stack
- **Clear**: Remove all elements from the stack

## Implementation Details

This implementation uses:
- **Vec<T>**: Dynamic array as the underlying data structure
- **Arc<Mutex<Vec<T>>>**: Thread-safe wrapper for concurrent access
- **Generic type T**: Supports any data type

## Time Complexity

- **Push**: O(1) amortized
- **Pop**: O(1)
- **Top**: O(1)
- **IsEmpty**: O(1)
- **Size**: O(1)
- **Clear**: O(1)

## Space Complexity

- **Overall**: O(n) where n is the number of elements

## Examples

**Basic Usage:**
```rust
let stack = Stack::new();
stack.push(1);
stack.push(2);
stack.push(3);

println!("Size: {}", stack.size()); // Size: 3
println!("Top: {:?}", stack.top()); // Top: Some(3)

if let Some(value) = stack.pop() {
    println!("Popped: {}", value); // Popped: 3
}
```

**Thread-Safe Operations:**
```rust
let stack = Stack::new();
let stack_clone = stack.clone();

// Safe to use across threads
thread::spawn(move || {
    stack_clone.push(42);
});
```

## Key Features

1. **Thread Safety**: Uses Arc<Mutex<>> for concurrent access
2. **Memory Safety**: Rust's ownership system prevents memory leaks
3. **Generic**: Works with any data type
4. **Clone Support**: Can be shared between threads
5. **Error Handling**: Graceful handling of mutex lock failures

## Use Cases

- Function call management (call stack)
- Expression evaluation
- Undo operations in text editors
- Backtracking algorithms
- Browser history navigation
- Balanced parentheses checking
