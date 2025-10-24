# Queue Data Structure (Go Implementation)

A queue is a linear data structure that follows the First In, First Out (FIFO) principle. Elements are added to the rear and removed from the front, like people waiting in line.

## Operations

- **Enqueue**: Add an element to the rear of the queue
- **Dequeue**: Remove and return the front element from the queue
- **Peek/Front**: View the front element without removing it
- **IsEmpty**: Check if the queue is empty
- **Size**: Get the number of elements in the queue
- **Clear**: Remove all elements from the queue

## Implementation Details

This implementation provides two approaches:

### 1. Simple Queue (`simple_queue.go`)
- **[]T**: Slice as the underlying data structure
- **sync.Mutex**: Thread-safe synchronization
- **Interface**: Defines the contract for queue operations
- **O(n) Dequeue**: Uses `data[1:]` which shifts all elements

### 2. Optimal Queue (`optimal_queue.go`)
- **Circular Buffer**: Efficient O(1) operations
- **Head/Tail pointers**: Track front and rear positions
- **Dynamic resizing**: Automatically grows when needed
- **O(1) all operations**: No element shifting required

## Time Complexity

### Simple Queue
- **Enqueue**: O(1) amortized
- **Dequeue**: O(n) - shifts all elements
- **Peek**: O(1)
- **IsEmpty**: O(1)
- **Size**: O(1)
- **Clear**: O(1)

### Optimal Queue
- **Enqueue**: O(1) amortized
- **Dequeue**: O(1)
- **Peek**: O(1)
- **IsEmpty**: O(1)
- **Size**: O(1)
- **Clear**: O(1)

## Space Complexity

- **Overall**: O(n) where n is the number of elements
- **Optimal Queue**: More memory efficient with circular buffer

## Examples

**Simple Queue Usage:**
```go
q := newQueue[int]()

q.Enqueue(1)
q.Enqueue(2)
q.Enqueue(3)

fmt.Println("Size:", q.Size()) // Size: 3

if val, ok := q.Peek(); ok {
    fmt.Println("Front:", val) // Front: 1
}

if val, ok := q.Dequeue(); ok {
    fmt.Println("Dequeued:", val) // Dequeued: 1
}
```

**Optimal Queue Usage:**
```go
q := NewOptimalQueue[int]()

q.Enqueue(1)
q.Enqueue(2)
q.Enqueue(3)

fmt.Println("All elements:", q.ToSlice()) // [1, 2, 3]
fmt.Println("Size:", q.Size()) // Size: 3

if val, ok := q.Peek(); ok {
    fmt.Println("Front:", val) // Front: 1
}
```

**Interface Usage:**
```go
var queue Queue[int] = newQueue[int]()
queue.Enqueue(42)
// All operations available through interface
```

## Key Features

1. **Thread Safety**: Uses sync.Mutex for concurrent access
2. **Interface Design**: Clean separation of interface and implementation
3. **Error Handling**: Returns (value, bool) for safe operations
4. **Memory Efficient**: Optimal queue uses circular buffer
5. **Go Idioms**: Follows Go conventions and patterns
6. **Generic Support**: Works with any data type using generics

## Implementation Comparison

| Feature | Simple Queue | Optimal Queue |
|---------|--------------|---------------|
| **Dequeue Complexity** | O(n) | O(1) |
| **Memory Usage** | Less efficient | More efficient |
| **Implementation** | Simpler | More complex |
| **Performance** | Slower for large queues | Faster for all operations |
| **Use Case** | Learning/Simple cases | Production/Performance |

## Use Cases

- Process scheduling (CPU, I/O)
- Breadth-First Search (BFS)
- Print job management
- Web server request handling
- Message queues
- Buffer management
- Task scheduling

## Go-Specific Features

- **Defer**: Automatic mutex unlocking with defer
- **Multiple Return Values**: (value, success) pattern
- **Interface Satisfaction**: Implements Queue interface
- **Zero Values**: Safe initialization with zero values
- **Generics**: Type-safe implementation with `[T any]`
- **Circular Buffer**: Efficient memory usage in optimal implementation

## Performance Considerations

### Simple Queue
- Good for: Learning, small datasets, simple use cases
- Avoid for: High-frequency operations, large datasets

### Optimal Queue
- Good for: Production systems, high-performance requirements
- Features: Automatic resizing, O(1) operations, memory efficient

## Best Practices

1. **Choose Implementation**: Use optimal queue for production
2. **Error Handling**: Always check the bool return value
3. **Thread Safety**: Safe for concurrent access
4. **Memory Management**: Optimal queue handles resizing automatically
5. **Interface Usage**: Use interface for flexibility
