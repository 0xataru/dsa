# Process String with Special Operations II - Go Solution

## Problem Summary
Same four operations as version I (append, pop `*`, duplicate `#`, reverse `%`), but the result may reach `10^15` characters. We must return only the `kth` character, so building the string is impossible.

## Solution: Length Tracking + Backward Walk

The key insight: we never need the string, only where index `k` *came from*. So:

1. **Forward pass** — compute `length[i]`, the length of `result` after the first `i` characters. Clamp every length at `10^15 + 1` so the doubling from `#` cannot overflow `uint64` while staying large enough for any valid `k`.
2. **Bounds check** — if `k >= length[n]`, the index does not exist, return `'.'`.
3. **Backward pass** — walk the operations in reverse, rewriting `idx` into the index it occupied *before* each operation:
   - letter: it sits at the last slot. If `idx == length[i]` (the length before appending), this is the source character — return it.
   - `*`: the removed character left no trace, `idx` is unchanged; the length was one larger before.
   - `#`: the second half mirrors the first, so `idx %= prevLen` folds it back into the original copy.
   - `%`: a reversal sends index `idx` to `curLen - 1 - idx`.

```go
const limit = uint64(1_000_000_000_000_000) + 1

func processStr(s string, k int64) byte {
    k0 := uint64(k)
    n := len(s)

    length := make([]uint64, n+1)
    for i := 0; i < n; i++ {
        switch c := s[i]; {
        case c >= 'a' && c <= 'z':
            length[i+1] = min(length[i]+1, limit)
        case c == '*':
            if length[i] > 0 {
                length[i+1] = length[i] - 1
            } else {
                length[i+1] = 0
            }
        case c == '#':
            length[i+1] = min(length[i]*2, limit)
        case c == '%':
            length[i+1] = length[i]
        }
    }

    curLen := length[n]
    if k0 >= curLen {
        return '.'
    }

    idx := k0
    for i := n - 1; i >= 0; i-- {
        switch c := s[i]; {
        case c >= 'a' && c <= 'z':
            prevLen := length[i]
            if idx == prevLen {
                return c
            }
            curLen = prevLen
        case c == '*':
            curLen++
        case c == '#':
            prevLen := length[i]
            if prevLen > 0 {
                idx %= prevLen
            }
            curLen = prevLen
        case c == '%':
            if curLen > 0 {
                idx = curLen - 1 - idx
            }
        }
    }

    return '.'
}
```

### Why the clamp matters
With up to `10^5` characters, a string of `#`s would double the length toward `2^(10^5)`. Clamping at `10^15 + 1` keeps arithmetic in `uint64` and is harmless: any clamped length is far above the largest valid `k` (`10^15`), so the bounds check and `% prevLen` reductions still behave correctly. (`min` is a built-in since Go 1.21.)

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) — one forward pass, one backward pass |
| Space | O(n) for the prefix-length array |
