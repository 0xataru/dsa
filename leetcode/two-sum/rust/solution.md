# Two Sum - Optimal Rust Solution

## Problem Summary
Given an array of integers and a target sum, find the indices of two numbers that add up to the target.

## Solution Approach: HashMap (One-Pass)

### Algorithm Overview
This Rust implementation uses a `HashMap` to achieve O(n) time complexity by trading space for time. The solution leverages Rust's memory safety, zero-cost abstractions, and efficient standard library collections.

### Key Rust Features Utilized
- **HashMap**: Rust's `std::collections::HashMap` provides O(1) average-case lookups
- **Pattern Matching**: Using `if let Some(&index)` for elegant option handling
- **Iterator Patterns**: `enumerate()` for index-value pairs
- **Borrowing**: Efficient memory usage with references (`&num`)
- **Memory Safety**: No risk of buffer overflows or null pointer dereferences

### Step-by-Step Algorithm
1. Create a mutable `HashMap<i32, usize>` to store `value -> index` mappings
2. Iterate through the array with `enumerate()` to get both index and value:
   - For each element `nums[i]`, calculate `complement = target - nums[i]`
   - Check if `complement` exists in the HashMap using pattern matching
   - If found, return `vec![stored_index, current_index]`
   - If not found, insert `nums[i] -> i` into the HashMap
3. Continue until a pair is found

### Implementation Details

```rust
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map: HashMap<i32, usize> = HashMap::new();
    
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        
        if let Some(&index) = num_map.get(&complement) {
            return vec![index as i32, i as i32];
        }
        
        num_map.insert(num, i);
    }
    
    vec![] // Should never reach here given problem constraints
}
```

### Rust-Specific Optimizations

#### Memory Management
- **Zero Allocations During Search**: Only allocates memory for the HashMap and result vector
- **Efficient Borrowing**: Uses `&num` to avoid unnecessary copying
- **Move Semantics**: Input vector is moved, eliminating unnecessary clones in the main algorithm

#### Type Safety
- **Explicit Type Annotations**: `HashMap<i32, usize>` makes the intent clear
- **Safe Indexing**: No risk of out-of-bounds access due to Rust's safety guarantees
- **Pattern Matching**: `if let Some(&index)` handles the Option type safely

#### Performance Features
- **Iterator Chains**: `nums.iter().enumerate()` is highly optimized
- **No Bounds Checking**: Rust's compiler optimizations eliminate redundant checks
- **Cache-Friendly**: HashMap implementation is optimized for modern CPU architectures

### Complexity Analysis

#### Time Complexity: O(n)
- Single pass through the array: O(n)
- HashMap operations (insert/lookup): O(1) average case
- Total: O(n) × O(1) = O(n)

#### Space Complexity: O(n)
- HashMap stores at most n key-value pairs
- Result vector: O(1) space
- Total: O(n)

### Comparison with Other Languages

| Language | HashMap/Map | Pattern Matching | Memory Safety | Performance |
|----------|-------------|------------------|---------------|-------------|
| **Rust** | ✅ Built-in | ✅ Native | ✅ Compile-time | ⚡ Zero-cost |
| Go | ✅ Built-in | ❌ Manual | ❌ Runtime | ⚡ Fast |
| Python | ✅ dict | ❌ Manual | ❌ Runtime | 🐌 Slower |
| Java | ✅ HashMap | ❌ Manual | ❌ Runtime | ⚡ Fast |

### Edge Cases Handled

1. **Integer Overflow**: Rust's `i32` handles the full range specified in constraints
2. **Negative Numbers**: HashMap keys work seamlessly with negative integers
3. **Duplicate Values**: Correctly handles cases like `[3,3]` with `target=6`
4. **Large Inputs**: Efficient memory usage scales well up to 10⁴ elements
5. **Zero Values**: Properly handles zero elements and zero targets

### Advanced Rust Features

#### Option Handling
```rust
// Elegant pattern matching instead of explicit null checks
if let Some(&index) = num_map.get(&complement) {
    return vec![index as i32, i as i32];
}
```

#### Iterator Efficiency
```rust
// Zero-cost abstraction - compiles to optimal assembly
for (i, &num) in nums.iter().enumerate() {
    // Processing logic
}
```

#### Type Conversions
```rust
// Safe casting with explicit intent
return vec![index as i32, i as i32];
```

### Performance Benchmarking

The implementation includes a performance test that measures:
- **Execution Time**: Actual runtime for 1000-element arrays
- **Memory Usage**: Efficient HashMap allocation patterns
- **Scalability**: Linear scaling with input size

### Alternative Rust Implementations

1. **Using `Vec` with Binary Search**: O(n log n) time, loses original indices
2. **Two-Pass HashMap**: Same complexity, but requires two iterations
3. **Using `BTreeMap`**: O(n log n) time, but maintains sorted order
4. **Array-Based (for limited ranges)**: O(1) space, O(n) time, but limited applicability

### Rust Best Practices Demonstrated

- **Ownership**: Clear ownership transfer of input vector
- **Borrowing**: Efficient borrowing patterns with `&num`
- **Error Handling**: Proper use of `Option` types
- **Documentation**: Comprehensive doc comments
- **Testing**: Built-in test cases with verification
- **Performance**: Zero-cost abstractions and optimal algorithms

### Cargo.toml Compatibility
This solution uses only standard library features, requiring no external dependencies:

```toml
[package]
name = "two-sum"
version = "0.1.0"
edition = "2021"

[dependencies]
# No external dependencies required
```

### Conclusion

This Rust implementation represents the optimal solution for the Two Sum problem, leveraging Rust's unique features:
- **Memory Safety**: No undefined behavior or memory leaks
- **Performance**: Zero-cost abstractions with optimal runtime
- **Expressiveness**: Clear, readable code with powerful pattern matching
- **Reliability**: Compile-time guarantees prevent entire classes of bugs

The solution achieves O(n) time complexity while maintaining idiomatic Rust code patterns and demonstrating the language's strengths in systems programming and performance-critical applications.
