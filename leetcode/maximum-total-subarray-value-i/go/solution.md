# Maximum Total Subarray Value I - Go Solution

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

```go
func maxTotalValue(nums []int, k int) int64 {
    minVal, maxVal := nums[0], nums[0]
    for _, num := range nums[1:] {
        if num < minVal {
            minVal = num
        }
        if num > maxVal {
            maxVal = num
        }
    }

    return int64(maxVal-minVal) * int64(k)
}
```

Use `int64` for the result to avoid overflow when `k` and values are large.

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(1) |

### Edge Cases
- Single element: `max - min = 0`, answer is `0`.
- All equal: same as above.
