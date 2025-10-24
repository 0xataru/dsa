# Queue Data Structure

A queue is a linear data structure that follows the First In, First Out (FIFO) principle. It's like a line of people waiting - the first person to arrive is the first to be served.

## How it Works

Think of a queue at a bank - you can only:
- **Enqueue**: Add a new person to the back of the line
- **Dequeue**: Remove the person from the front of the line
- **Peek/Front**: Look at the person at the front without removing them

## Operations

| Operation | Description | Time Complexity | Space Complexity |
|-----------|-------------|-----------------|------------------|
| `enqueue(item)` | Add element to rear | O(1) | O(1) |
| `dequeue()` | Remove and return front element | O(1) | O(1) |
| `front()` / `peek()` | View front element without removing | O(1) | O(1) |
| `isEmpty()` | Check if queue is empty | O(1) | O(1) |
| `size()` | Get number of elements | O(1) | O(1) |

## Visual Example

```
Initial:     Enqueue(1):    Enqueue(2):    Enqueue(3):    Dequeue():
   []           [1]           [1,2]          [1,2,3]        [2,3]
                                                             (returns 1)

Result: Queue contains [2,3] with 2 at front
```

## Common Use Cases

### 1. **Process Scheduling**
```rust
// CPU scheduling: Process A -> Process B -> Process C
// Queue: [A, B, C] <- A runs first, then B, then C
```

### 2. **Breadth-First Search (BFS)**
```rust
// Graph traversal: Visit nodes level by level
// Queue: [start] -> [neighbors] -> [neighbors of neighbors]
```

### 3. **Print Job Management**
```rust
// Printer queue: "Document1" -> "Document2" -> "Document3"
// Queue: ["Doc1", "Doc2", "Doc3"] <- prints in order
```

### 4. **Web Server Request Handling**
```rust
// HTTP requests: GET /api -> POST /data -> PUT /update
// Queue: [GET, POST, PUT] <- processes in order
```

### 5. **Message Queues**
```rust
// Chat messages: "Hello" -> "How are you?" -> "Goodbye"
// Queue: ["Hello", "How are you?", "Goodbye"]
```

## Implementation Approaches

### 1. **Array-based Queue (Simple)**
```rust
struct Queue<T> {
    data: Vec<T>,
}
```
- **Pros**: Simple, cache-friendly
- **Cons**: O(n) dequeue (shifts all elements)

### 2. **Circular Buffer Queue (Optimal)**
```rust
struct Queue<T> {
    data: Vec<T>,
    head: usize,
    tail: usize,
    size: usize,
}
```
- **Pros**: O(1) all operations, memory efficient
- **Cons**: More complex implementation

### 3. **Linked List Queue**
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
let queue = Arc::new(Mutex::new(Vec::new()));
```

### Go Implementation
```go
// Thread-safe with sync.Mutex
type Queue struct {
    mu   sync.Mutex
    data []int
}
```

## Common Patterns

### 1. **BFS (Breadth-First Search)**
```rust
fn bfs(graph: &Graph, start: Node) -> Vec<Node> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(node) = queue.pop_front() {
        for neighbor in graph.neighbors(node) {
            if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }
    visited.into_iter().collect()
}
```

### 2. **Level Order Traversal**
```rust
fn level_order_traversal(root: Option<&TreeNode>) -> Vec<Vec<i32>> {
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    
    if let Some(node) = root {
        queue.push_back(node);
    }
    
    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level = Vec::new();
        
        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                level.push(node.val);
                if let Some(left) = &node.left {
                    queue.push_back(left);
                }
                if let Some(right) = &node.right {
                    queue.push_back(right);
                }
            }
        }
        result.push(level);
    }
    result
}
```

### 3. **Sliding Window Maximum**
```rust
fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    
    for i in 0..nums.len() {
        // Remove elements outside window
        while !queue.is_empty() && *queue.front().unwrap() < i as i32 - k + 1 {
            queue.pop_front();
        }
        
        // Remove smaller elements
        while !queue.is_empty() && nums[*queue.back().unwrap() as usize] < nums[i] {
            queue.pop_back();
        }
        
        queue.push_back(i as i32);
        
        if i >= k as usize - 1 {
            result.push(nums[*queue.front().unwrap() as usize]);
        }
    }
    result
}
```

## Advantages

- **Fair**: First come, first served
- **Simple**: Easy to understand and implement
- **Fast**: O(1) operations with proper implementation
- **Versatile**: Many applications in computer science

## Disadvantages

- **Limited Access**: Can only access front and rear
- **No Random Access**: Can't access middle elements
- **Size Limitations**: Array-based queues have fixed size

## When to Use

✅ **Use Queue when:**
- Need FIFO behavior
- Implementing BFS
- Task scheduling
- Message processing
- Buffer management

❌ **Don't use Queue when:**
- Need random access to elements
- Need LIFO behavior (use stack instead)
- Need to search elements frequently

## Related Data Structures

- **Stack**: LIFO (Last In, First Out)
- **Deque**: Double-ended queue
- **Priority Queue**: Elements with priorities
- **Circular Buffer**: Fixed-size queue

## Practice Problems

1. **Binary Tree Level Order Traversal** - BFS with queue
2. **Sliding Window Maximum** - Deque for optimization
3. **Course Schedule** - Topological sort with queue
4. **Rotting Oranges** - BFS simulation
5. **Word Ladder** - BFS for shortest path
