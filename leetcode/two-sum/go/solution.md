# Two Sum - Optimal Solution

## Problem Summary
Given an array of integers and a target sum, find the indices of two numbers that add up to the target.

## Solution Approach: Hash Map (One-Pass)

### Algorithm Overview
The optimal solution uses a hash map to achieve O(n) time complexity by trading space for time. Instead of checking every pair of numbers (which would be O(n²)), we store each number and its index in a hash map as we iterate through the array.

### Key Insight
For each number `nums[i]`, we need to find if `target - nums[i]` exists in the array. By using a hash map, we can check this in O(1) time.

### Step-by-Step Algorithm
1. Create an empty hash map to store `value -> index` mappings
2. Iterate through the array once:
   - For each element `nums[i]`, calculate `complement = target - nums[i]`
   - Check if `complement` exists in the hash map
   - If it exists, return `[map[complement], i]`
   - If not, store `nums[i] -> i` in the hash map
3. Continue until a pair is found

### Implementation Details

```go
func twoSum(nums []int, target int) []int {
    numMap := make(map[int]int)
    
    for i, num := range nums {
        complement := target - num
        
        if index, exists := numMap[complement]; exists {
            return []int{index, i}
        }
        
        numMap[num] = i
    }
    
    return nil // Should never reach here given problem constraints
}
```

### Why This Works
- **Correctness**: When we find a complement in the map, we know that `nums[index] + nums[i] = target`
- **Uniqueness**: The problem guarantees exactly one solution, so we can return immediately when found
- **Order Independence**: We return indices in the order we find them, which satisfies the problem requirements

### Complexity Analysis

#### Time Complexity: O(n)
- We traverse the array exactly once
- Each hash map lookup and insertion takes O(1) average time
- Total: O(n) × O(1) = O(n)

#### Space Complexity: O(n)
- In the worst case, we store all n elements in the hash map
- This happens when the solution is the last two elements we check

### Comparison with Brute Force

| Approach | Time Complexity | Space Complexity | Description |
|----------|----------------|------------------|-------------|
| Brute Force | O(n²) | O(1) | Check all pairs |
| Hash Map | O(n) | O(n) | One-pass with hash map |

### Edge Cases Handled
1. **Negative numbers**: Works correctly as hash map handles any integer
2. **Zero values**: Properly handles zeros and negative targets
3. **Duplicate values**: When target is double a value (e.g., [3,3], target=6)
4. **Large arrays**: Scales efficiently up to the constraint limit (10⁴ elements)

### Test Cases Coverage
The implementation includes comprehensive test cases:
- Basic examples from the problem statement
- Negative numbers
- Zero target
- Duplicate values
- Edge cases within constraints

### Memory Optimization Notes
- Go's `map[int]int` is efficient for this use case
- We could optimize further for specific constraints (e.g., using arrays for limited ranges)
- Current solution prioritizes readability and general applicability

### Alternative Solutions Considered

1. **Brute Force O(n²)**: Simple but inefficient for large inputs
2. **Two-Pointer with Sorting**: O(n log n), but loses original indices
3. **Hash Map Two-Pass**: O(n) time, same space, but makes two passes
4. **Hash Map One-Pass**: ✅ **Chosen** - optimal balance of time and space

This solution represents the optimal approach for the Two Sum problem, achieving the best possible time complexity while maintaining reasonable space usage.
