# Stack Data Structure

A stack is a linear data structure that follows the Last In, First Out (LIFO) principle. It's like a stack of plates where you can only add or remove plates from the top.

## How it Works

Think of a stack of books - you can only:
- **Push**: Add a new book on top
- **Pop**: Remove the top book
- **Peek/Top**: Look at the top book without removing it

## Operations

| Operation | Description | Time Complexity | Space Complexity |
|-----------|-------------|-----------------|------------------|
| `push(item)` | Add element to top | O(1) | O(1) |
| `pop()` | Remove and return top element | O(1) | O(1) |
| `top()` / `peek()` | View top element without removing | O(1) | O(1) |
| `isEmpty()` | Check if stack is empty | O(1) | O(1) |
| `size()` | Get number of elements | O(1) | O(1) |

## Visual Example

```
Initial:     Push(1):     Push(2):     Push(3):     Pop():
   []           [1]         [2]          [3]         [2]
                           [1]          [2]         [1]
                                       [1]

Result: Stack contains [1, 2] with 2 on top
```

## Common Use Cases

### 1. **Function Call Management**
```rust
// When function A calls function B
// Stack: [A, B] <- B is on top
// When B returns, A continues
```

### 2. **Expression Evaluation**
```rust
// Evaluate: 2 + 3 * 4
// Stack: [2, 3, 4, *] -> [2, 12, +] -> [14]
```

### 3. **Undo Operations**
```rust
// Text editor: "Hello" -> "Hello World" -> "Hello"
// Stack: ["Hello", "Hello World"] <- can undo to "Hello"
```

### 4. **Balanced Parentheses**
```rust
// Check: "((()))" is balanced
// Stack: ['(', '(', '('] -> pop on ')'
```

### 5. **Browser History**
```rust
// Navigate: Google -> Stack Overflow -> GitHub
// Stack: [Google, Stack Overflow, GitHub]
// Back button pops GitHub
```

## Implementation Approaches

### 1. **Array-based Stack**
```rust
struct Stack<T> {
    data: Vec<T>,
}
```
- **Pros**: Cache-friendly, simple
- **Cons**: Fixed size (unless dynamic), memory waste

### 2. **Linked List Stack**
```rust
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
```
- **Pros**: Dynamic size, no memory waste
- **Cons**: Extra memory for pointers, cache misses

## Thread Safety

### Rust Implementation
```rust
// Thread-safe with Arc<Mutex<Vec<T>>>
let stack = Arc::new(Mutex::new(Vec::new()));
```

### Go Implementation
```go
// Thread-safe with sync.Mutex
type Stack struct {
    mu   sync.Mutex
    data []int
}
```

## Common Patterns

### 1. **Matching Brackets**
```rust
fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                if stack.pop() != Some(matching_bracket(c)) {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
```

### 2. **Next Greater Element**
```rust
fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::new();
    let mut result = vec![-1; nums.len()];
    
    for i in 0..nums.len() {
        while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[i] {
            let idx = stack.pop().unwrap();
            result[idx] = nums[i];
        }
        stack.push(i);
    }
    result
}
```

## Advantages

- **Simple**: Easy to understand and implement
- **Fast**: O(1) operations
- **Memory Efficient**: Only stores what's needed
- **Versatile**: Many applications

## Disadvantages

- **Limited Access**: Can only access top element
- **No Random Access**: Can't access middle elements
- **Size Limitations**: Array-based stacks have fixed size

## When to Use

✅ **Use Stack when:**
- Need LIFO behavior
- Implementing recursion
- Parsing expressions
- Undo/redo functionality
- Backtracking algorithms

❌ **Don't use Stack when:**
- Need random access to elements
- Need FIFO behavior (use queue instead)
- Need to search elements frequently

## Related Data Structures

- **Queue**: FIFO (First In, First Out)
- **Deque**: Double-ended queue
- **Priority Queue**: Elements with priorities
- **Heap**: Tree-based priority structure

## Practice Problems

1. **Valid Parentheses** - Check if brackets are balanced
2. **Daily Temperatures** - Find next warmer day
3. **Largest Rectangle in Histogram** - Find largest rectangle
4. **Min Stack** - Stack that tracks minimum element
5. **Evaluate Reverse Polish Notation** - Calculate expressions
