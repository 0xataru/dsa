# Total Waviness of Numbers in Range I - Rust Solution

## Problem Summary
For each integer in `[num1, num2]`, count digits that are strict peaks or valleys (excluding first and last digit). Return the sum over the whole range.

## Solution Approach: Brute Force

### Algorithm
1. Iterate `num` from `num1` to `num2` inclusive
2. Convert `num` to digits
3. If fewer than 3 digits, skip (waviness 0)
4. For each inner index `i` (not first/last), check:
   - **Peak**: `digits[i] > digits[i-1]` and `digits[i] > digits[i+1]`
   - **Valley**: `digits[i] < digits[i-1]` and `digits[i] < digits[i+1]`
5. Add counts to the running total

### Implementation

```rust
fn total_waviness(num1: i32, num2: i32) -> i32 {
    let mut waviness = 0;

    for num in num1..=num2 {
        let digits: Vec<u8> = num
            .to_string()
            .bytes()
            .map(|b| b - b'0')
            .collect();

        if digits.len() < 3 {
            continue;
        }

        for i in 1..digits.len() - 1 {
            let prev = digits[i - 1];
            let curr = digits[i];
            let next = digits[i + 1];

            if (curr > prev && curr > next) || (curr < prev && curr < next) {
                waviness += 1;
            }
        }
    }

    waviness
}
```

### Complexity Analysis

| Metric | Value |
|--------|-------|
| Time | O((num2 − num1 + 1) × d), where d ≤ 6 for `num2 ≤ 10^5` |
| Space | O(d) per number for the digit vector |

With `num2 - num1 ≤ 10^5` and at most 6 digits, this easily fits the constraints.

### Why Brute Force Is Enough
- Range size is at most 10⁵
- Each number has at most 6 digits
- No need for digit DP unless constraints grow to 10¹⁸+
