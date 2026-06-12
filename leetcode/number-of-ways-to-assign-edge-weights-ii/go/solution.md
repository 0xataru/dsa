# Number of Ways to Assign Edge Weights II - Go Solution

## Problem Summary
For each query `(u, v)`, count assignments of weights 1/2 on the unique `u`-`v` path such that total cost is odd.

## Solution Approach: LCA + Power of Two

### Key Insight
On a path with `len` edges:

```
answer = 2^(len - 1)   if len > 0
answer = 0             if u == v
```

### Path Length via LCA
```
len(u, v) = depth[u] + depth[v] - 2 * depth[LCA(u, v)]
```

Use **binary lifting** for O(log n) LCA per query.

### Algorithm
1. DFS from node 1 → `depth`, `up[0]`
2. Build binary lifting table
3. Precompute `pow2[i] = 2^i mod (10^9 + 7)`
4. Answer each query via LCA and `pow2[len - 1]`

### Complexity

| Metric | Value |
|--------|-------|
| Preprocess | O(n log n) |
| Per query | O(log n) |
| Total | O((n + q) log n) |
