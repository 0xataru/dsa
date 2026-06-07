# Left and Right Sum Differences - Rust Solution

## Problem Summary
For each index `i`, return `|sum(nums[0..i]) - sum(nums[i+1..n])|`.

## Solution Approach: Total Sum + Running Left Sum

### Key Insight
```
rightSum[i] = totalSum - leftSum[i] - nums[i]
answer[i]   = |leftSum[i] - rightSum[i]|
```

No need to build `leftSum` and `rightSum` arrays separately.

### Algorithm
1. Compute `total_sum` once
2. Iterate with `left_sum = 0`
3. For each `num`:
   - `right_sum = total_sum - left_sum - num`
   - push `|left_sum - right_sum|` to answer
   - `left_sum += num`

### Implementation

```rust
fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let total_sum: i32 = nums.iter().sum();
    let mut left_sum = 0;
    let mut answer = Vec::with_capacity(nums.len());

    for &num in &nums {
        let right_sum = total_sum - left_sum - num;
        answer.push((left_sum - right_sum).abs());
        left_sum += num;
    }

    answer
}
```

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(1) extra (output excluded) |

### Alternative
Two-pass with explicit prefix/suffix arrays — same complexity, more memory.
