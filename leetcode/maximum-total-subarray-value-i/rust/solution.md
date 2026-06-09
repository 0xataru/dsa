# Maximum Total Subarray Value I - Rust Solution

## Problem Summary
Choose exactly `k` subarrays (with repetition allowed). Maximize the sum of `(max - min)` over each chosen subarray.

## Solution Approach: Global Min/Max

### Key Insight
The largest possible value of any single subarray is:

```
global_max(nums) - global_min(nums)
```

Any subarray containing both the global minimum and global maximum achieves this value. Since subarrays can overlap and repeat, pick this optimal subarray `k` times:

```
answer = (max(nums) - min(nums)) * k
```

### Why This Works
- No subarray can have `max - min` greater than the array's global range.
- A subarray spanning from the index of `min` to the index of `max` (in either order) attains that range.
- Repeating the same optimal subarray `k` times is allowed.

### Implementation

```rust
fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let min_val = *nums.iter().min().unwrap() as i64;
    let max_val = *nums.iter().max().unwrap() as i64;

    (max_val - min_val) * k as i64
}
```

Use `i64` for the result to avoid overflow when `k` and values are large.

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(1) |

### Edge Cases
- Single element: `max - min = 0`, answer is `0`.
- All equal: same as above.
