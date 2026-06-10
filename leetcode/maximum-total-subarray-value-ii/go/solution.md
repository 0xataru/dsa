# Maximum Total Subarray Value II - Go Solution

## Problem Summary
Pick exactly `k` **distinct** subarrays. Maximize the sum of `(max - min)` over each.

Unlike Part I, the same `(l, r)` cannot be chosen twice.

## Solution Approach: Sparse Table + Greedy Heap

### Key Insight
For a fixed left endpoint `l`, as we shrink `r` leftward, the subarray value is **non-increasing**.

So for each `l`, candidates are: `[l, n-1], [l, n-2], ..., [l, l]`.

### Algorithm
1. Build sparse tables for range `max` and `min`
2. Push `(value(l, n-1), l, n-1)` for every `l` into a max-heap
3. Repeat `k` times:
   - Pop the largest `(val, l, r)`
   - Add `val` to answer
   - If `r > l`, push `(value(l, r-1), l, r-1)`

### Complexity

| Metric | Value |
|--------|-------|
| Preprocess | O(n log n) |
| Greedy | O(k log n) |
| Space | O(n log n) |

### Part I vs Part II

| | Part I | Part II |
|---|--------|---------|
| Distinct `(l, r)` | No | Yes |
| Answer | `(max - min) * k` | Greedy top-k |
