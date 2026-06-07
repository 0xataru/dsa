# Left and Right Sum Differences - Go Solution

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
1. Compute `totalSum` once
2. Iterate with `leftSum = 0`
3. For each `num`:
   - `rightSum = totalSum - leftSum - num`
   - append `|leftSum - rightSum|` to answer
   - `leftSum += num`

### Implementation

```go
func leftRightDifference(nums []int) []int {
    totalSum := 0
    for _, num := range nums {
        totalSum += num
    }

    leftSum := 0
    answer := make([]int, 0, len(nums))

    for _, num := range nums {
        rightSum := totalSum - leftSum - num
        diff := leftSum - rightSum
        if diff < 0 {
            diff = -diff
        }
        answer = append(answer, diff)
        leftSum += num
    }

    return answer
}
```

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(1) extra (output excluded) |

### Alternative
Two-pass with explicit prefix/suffix arrays — same complexity, more memory.
