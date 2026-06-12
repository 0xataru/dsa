# Number of Ways to Assign Edge Weights I - Rust Solution

## Problem Summary
On the path from root (node 1) to any deepest node `x`, assign each edge weight 1 or 2. Count assignments where total path cost is odd.

## Solution Approach: Max Depth + Power of Two

### Key Insight
The path from root to a deepest node has `d` edges, where `d = max_depth` (depth counted from 0 at root).

Each edge weight is 1 (odd) or 2 (even). Total cost is odd iff an **odd number** of edges have weight 1.

For `d` edges, the number of assignments with odd sum is:

```
2^(d - 1)
```

Proof: half of all `2^d` binary assignments have an odd number of 1s.

Which deepest node we pick does not matter — all root-to-deepest paths have the same length `d`.

### Algorithm
1. Build adjacency list
2. DFS/BFS from node 1 to find `max_depth`
3. Return `2^(max_depth - 1) mod (10^9 + 7)` via fast exponentiation

### Implementation

```rust
fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
    // ... build graph, find max_depth ...

    mod_pow(2, (max_depth - 1) as i64, MOD) as i32
}
```

### Examples

| Tree | max_depth | Edges on path | Answer |
|------|-----------|---------------|--------|
| 1—2 | 1 | 1 | 2^0 = 1 |
| 1—3—4 | 2 | 2 | 2^1 = 2 |
| 1—2—3—4 | 3 | 3 | 2^2 = 4 |

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(n) |
