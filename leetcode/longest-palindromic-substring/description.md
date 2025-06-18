# Longest Palindromic Substring

Given a string `s`, return the longest palindromic substring in `s`.

A palindrome is a string that reads the same forward and backward.

## Examples

**Example 1:**
```
Input: s = "babad"
Output: "bab"
Explanation: "aba" is also a valid answer.
```

**Example 2:**
```
Input: s = "cbbd"
Output: "bb"
```

## Constraints

- `1 <= s.length <= 1000`
- `s` consist of only digits and English letters.

## Solution Approaches

This repository contains **two different implementations**:

### 1. Expand Around Centers - O(n²)
- **Time Complexity**: O(n²)
- **Space Complexity**: O(1)  
- **Pros**: Simple to understand and implement
- **Cons**: Quadratic time complexity
- **Best for**: Small to medium inputs (n ≤ 5,000), coding interviews

### 2. Manacher's Algorithm - O(n)
- **Time Complexity**: O(n)
- **Space Complexity**: O(n)
- **Pros**: Optimal linear time complexity
- **Cons**: Complex algorithm, harder to understand
- **Best for**: Large inputs (n > 5,000), performance-critical applications

## Performance Comparison

Based on benchmarks with both Rust and Go implementations:

| Input Size | O(n²) Time | O(n) Time | Speedup |
|------------|------------|-----------|---------|
| 100 | ~0.01ms | ~0.01ms | ~1.2x |
| 1,000 | ~0.15ms | ~0.02-0.24ms | 1.5-8x |
| 5,000 | ~4-5ms | ~0.1-3ms | 1.8-42x |
| 10,000 | ~15-17ms | ~0.15-7ms | 2.4-102x |

**Note**: For LeetCode constraints (n ≤ 1,000), both approaches perform well, but the O(n²) solution is preferred for its simplicity.