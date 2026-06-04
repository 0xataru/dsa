# Total Waviness of Numbers in Range I - Go Solution

## Problem Summary
For each integer in `[num1, num2]`, count digits that are strict peaks or valleys (excluding first and last digit). Return the sum over the whole range.

## Solution Approach: Brute Force

### Algorithm
1. Iterate `num` from `num1` to `num2` inclusive
2. Convert `num` to a digit string
3. If fewer than 3 digits, skip (waviness 0)
4. For each inner index, check peak or valley conditions
5. Add counts to the running total

### Implementation

```go
func totalWaviness(num1, num2 int) int {
    waviness := 0
    for num := num1; num <= num2; num++ {
        s := strconv.Itoa(num)
        if len(s) < 3 {
            continue
        }
        for i := 1; i < len(s)-1; i++ {
            prev := int(s[i-1] - '0')
            curr := int(s[i] - '0')
            next := int(s[i+1] - '0')
            if (curr > prev && curr > next) || (curr < prev && curr < next) {
                waviness++
            }
        }
    }
    return waviness
}
```

### Complexity Analysis

| Metric | Value |
|--------|-------|
| Time | O((num2 − num1 + 1) × d), where d ≤ 6 for `num2 ≤ 10^5` |
| Space | O(d) per number for the digit string |

With `num2 - num1 ≤ 10^5` and at most 6 digits, this easily fits the constraints.

### Why Brute Force Is Enough
- Range size is at most 10⁵
- Each number has at most 6 digits
- No need for digit DP unless constraints grow to 10¹⁸+
