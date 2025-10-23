# Stack Data Structure (Go Implementation)

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
- **[]int**: Slice as the underlying data structure
- **sync.Mutex**: Thread-safe synchronization
- **Interface**: Defines the contract for stack operations
- **Struct**: Encapsulates data and mutex

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
```go
s := new(stack)

s.Push(1)
s.Push(2)
s.Push(3)

fmt.Println("Size:", s.Size()) // Size: 3

if val, ok := s.Top(); ok {
    fmt.Println("Top:", val) // Top: 3
}

if val, ok := s.Pop(); ok {
    fmt.Println("Popped:", val) // Popped: 3
}
```

**Interface Usage:**
```go
var stack Stack = new(stack)
stack.Push(42)
// All operations available through interface
```

## Key Features

1. **Thread Safety**: Uses sync.Mutex for concurrent access
2. **Interface Design**: Clean separation of interface and implementation
3. **Error Handling**: Returns (value, bool) for safe operations
4. **Memory Efficient**: Slice-based implementation
5. **Go Idioms**: Follows Go conventions and patterns

## Use Cases

- Function call management (call stack)
- Expression evaluation
- Undo operations in text editors
- Backtracking algorithms
- Browser history navigation
- Balanced parentheses checking

## Go-Specific Features

- **Defer**: Automatic mutex unlocking with defer
- **Multiple Return Values**: (value, success) pattern
- **Interface Satisfaction**: Implements Stack interface
- **Zero Values**: Safe initialization with zero values
