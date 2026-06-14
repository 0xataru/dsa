# Maximum Twin Sum of a Linked List - Rust Solution

## Problem Summary
For an even-length list, node `i` is twin of node `n-1-i`. Return the maximum sum of twin pairs.

## Solution 1: Copy to Vector

```rust
fn pair_sum_vec(head: Option<Box<ListNode>>) -> i32 {
    let mut nums = Vec::new();
    // ... collect values ...
    for i in 0..n / 2 {
        ans = ans.max(nums[i] + nums[n - 1 - i]);
    }
    ans
}
```

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(n) |

## Solution 2: Split + Reverse Second Half

```rust
// 1. count length
// 2. advance mutable ref to middle, take() second half
// 3. reverse second half
// 4. walk both halves in parallel
```

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(1) extra |

## Why LeetCode Ranks Vec *Faster* Despite O(n) Space

Asymptotically Solution 2 uses less memory, but in practice Solution 1 often wins on LeetCode benchmarks:

| Factor | Vec | In-place reverse |
|--------|-----|------------------|
| Passes over list | 1 (collect) | 3 (count, split, compare) + reverse loop |
| Memory access | contiguous array, cache-friendly | scattered `Box` nodes on heap |
| Rust overhead | simple `push` + index | `take()`, `Box` moves, mutable ref walk to midpoint |
| Allocations | one `Vec` growth | many pointer swaps during reverse |

### Rust-specific cost of reverse

LeetCode's `ListNode` is `Option<Box<ListNode>>`. Reversing means:
- `node.next.take()` — move ownership out of each node
- reassign `node.next = prev` — another move per node

That is **more work per node** than copying an `i32` into a vector.

### Is Solution 2 wrong?

No — it is **correct**. It is just slower in constant factors for typical input sizes (`n <= 10^5`).

### When to use which

| Approach | Prefer when |
|----------|-------------|
| Vec | Interview default, best runtime on LeetCode, simplest code |
| In-place | Memory is strictly limited, or list must not use extra array |

### Possible micro-optimization for in-place

Replace the length-count pass with **slow/fast pointers** to find the middle in one walk — still O(n) time, but one fewer pass. Reverse cost remains.

## Complexity Summary

Both solutions are O(n) time. Vec uses O(n) space; in-place uses O(1) extra but higher constant factor in Rust.
