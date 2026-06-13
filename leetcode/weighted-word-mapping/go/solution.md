# Weighted Word Mapping - Go Solution

## Problem Summary
For each word, sum character weights, take `mod 26`, map to a letter in reverse alphabetical order (`0 → 'z'`, `25 → 'a'`). Concatenate results.

## Solution Approach

```go
sum := 0
for _, ch := range word {
    sum += weights[ch-'a']
}
mapped := byte('z' - (sum % 26))
```

### Mapping Formula

| remainder | letter |
|-----------|--------|
| 0 | z |
| 1 | y |
| 8 | r |
| 25 | a |

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(total characters) |
| Space | O(1) extra |

Two variants included: explicit loop and pre-sized result slice — same logic.
