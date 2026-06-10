# Maximum Total Subarray Value II - Rust Solution

## Problem Summary
Pick exactly `k` **distinct** subarrays. Maximize the sum of `(max - min)` over each.

Unlike Part I, the same `(l, r)` cannot be chosen twice.

## Solution Approach: Sparse Table + Greedy Heap

### Key Insight
For a fixed left endpoint `l`, as we shrink `r` leftward, the subarray value is **non-increasing** (removing elements cannot increase `max - min`).

So for each `l`, the best subarrays are:
```
[l, n-1], [l, n-2], ..., [l, l]
```

### Algorithm
1. Build sparse tables for range `max` and `min` — O(n log n) preprocess, O(1) query
2. Push `(value(l, n-1), l, n-1)` for every `l` into a max-heap
3. Repeat `k` times:
   - Pop the largest `(val, l, r)`
   - Add `val` to answer
   - If `r > l`, push `(value(l, r-1), l, r-1)` — next best distinct subarray with same `l`

Each `(l, r)` is generated at most once; we always take the globally largest remaining value.

### Why Greedy Works
Values for fixed `l` form a decreasing sequence. The heap always exposes the best unseen subarray across all `l`.

### Complexity

| Metric | Value |
|--------|-------|
| Preprocess | O(n log n) |
| Greedy | O(k log n) |
| Space | O(n log n) |

With `n <= 5 * 10^4` and `k <= 10^5`, this fits comfortably.

### Part I vs Part II

| | Part I | Part II |
|---|--------|---------|
| Distinct `(l, r)` | No (repeat allowed) | Yes |
| Answer | `(max - min) * k` | Greedy top-k |
