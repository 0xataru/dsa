# Process String with Special Operations I - Go Solution

## Problem Summary
Simulate four operations on a result string: append a letter, pop the last char (`*`), duplicate (`#`), reverse (`%`).

## Solution: Direct Simulation

```go
func processStr(s string) string {
    result := []byte{}

    for _, ch := range s {
        switch ch {
        case '*':
            if len(result) > 0 {
                result = result[:len(result)-1]
            }
        case '#':
            result = append(result, result...)
        case '%':
            for i, j := 0, len(result)-1; i < j; i, j = i+1, j-1 {
                result[i], result[j] = result[j], result[i]
            }
        default:
            result = append(result, byte(ch))
        }
    }

    return string(result)
}
```

### Steps
1. Use a `[]byte` slice so pop, append, and in-place reverse are O(1) or O(n) without extra allocations
2. `*` — reslice to drop the last byte
3. `#` — `append(result, result...)` doubles the slice in one call
4. `%` — two-pointer swap in place
5. Otherwise — append the byte

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n · 2^k), k = number of `#` ops |
| Space | O(n · 2^k) for the result buffer |