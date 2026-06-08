# Partition Array According to Given Pivot - Go Solution

## Problem Summary
Rearrange the array so that elements `< pivot` come first, then `== pivot`, then `> pivot`, while preserving relative order within the less-than and greater-than groups.

## Solution Approach: Three Buckets

### Key Insight
A single pass into three slices preserves stability:
- `left` — elements `< pivot` (in original order)
- `equal` — elements `== pivot`
- `right` — elements `> pivot` (in original order)

Concatenate: `left + equal + right`.

### Implementation

```go
func pivotArray(nums []int, pivot int) []int {
    left := make([]int, 0, len(nums))
    equal := make([]int, 0)
    right := make([]int, 0)

    for _, num := range nums {
        switch {
        case num < pivot:
            left = append(left, num)
        case num == pivot:
            equal = append(equal, num)
        default:
            right = append(right, num)
        }
    }

    result := make([]int, 0, len(nums))
    result = append(result, left...)
    result = append(result, equal...)
    result = append(result, right...)
    return result
}
```

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(n) |

### Alternative
Two-pointer in-place partition (Dutch national flag) does **not** preserve relative order of `<` and `>` groups — three buckets is the straightforward stable approach.
