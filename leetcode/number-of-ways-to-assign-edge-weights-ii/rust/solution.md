# Number of Ways to Assign Edge Weights II - Rust Solution

## Problem Summary
For each query `(u, v)`, count assignments of weights 1/2 on the unique `u`-`v` path such that total cost is odd.

## Solution Approach: LCA + Power of Two

### Key Insight (same as Part I)
On a path with `len` edges, odd total cost requires an odd number of weight-1 edges:

```
answer = 2^(len - 1)   if len > 0
answer = 0             if len == 0  (same node, no edges)
```

### Path Length via LCA
```
len(u, v) = depth[u] + depth[v] - 2 * depth[LCA(u, v)]
```

Preprocess with **binary lifting** for O(log n) LCA per query.

### Algorithm
1. Build adjacency list, DFS from node 1 for `depth` and `up[0]`
2. Fill binary lifting table `up[k][v]`
3. Precompute `pow2[i] = 2^i mod (10^9 + 7)`
4. For each query: LCA → `len` → `pow2[len - 1]` or `0`

### Complexity

| Metric | Value |
|--------|-------|
| Preprocess | O(n log n) |
| Per query | O(log n) |
| Total | O((n + q) log n) |
| Space | O(n log n) |

### Part I vs Part II

| | Part I | Part II |
|---|--------|---------|
| Query | Deepest node from root | Arbitrary `(u, v)` |
| Path length | `max_depth` | LCA formula |
| Queries | 1 implicit | up to 10^5 |
