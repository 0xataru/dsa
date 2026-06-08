# Partition Array According to Given Pivot - Rust Solution

## Problem Summary
Rearrange the array so that elements `< pivot` come first, then `== pivot`, then `> pivot`, while preserving relative order within the less-than and greater-than groups.

## Solution Approach: Three Buckets

### Key Insight
A single pass into three vectors preserves stability:
- `left` — elements `< pivot` (in original order)
- `equal` — elements `== pivot`
- `right` — elements `> pivot` (in original order)

Concatenate: `left + equal + right`.

### Implementation

```rust
fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut left = Vec::new();
    let mut equal = Vec::new();
    let mut right = Vec::new();

    for num in nums {
        if num < pivot {
            left.push(num);
        } else if num == pivot {
            equal.push(num);
        } else {
            right.push(num);
        }
    }

    left.extend(equal);
    left.extend(right);
    left
}
```

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(n) |

### Alternative
Two-pointer in-place partition (Dutch national flag) does **not** preserve relative order of `<` and `>` groups — three buckets is the straightforward stable approach.
