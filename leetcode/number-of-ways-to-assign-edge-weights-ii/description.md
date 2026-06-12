# Number of Ways to Assign Edge Weights II

There is an undirected tree with `n` nodes labeled from 1 to `n`, rooted at node 1. The tree is represented by a 2D integer array `edges` of length `n - 1`, where `edges[i] = [ui, vi]` indicates that there is an edge between nodes `ui` and `vi`.

Initially, all edges have a weight of 0. You must assign each edge a weight of either 1 or 2.

The cost of a path between any two nodes `u` and `v` is the total weight of all edges in the path connecting them.

You are given a 2D integer array `queries`. For each `queries[i] = [ui, vi]`, determine the number of ways to assign weights to edges in the path such that the cost of the path between `ui` and `vi` is odd.

Return an array `answer`, where `answer[i]` is the number of valid assignments for `queries[i]`.

Since the answer may be large, apply modulo `10^9 + 7` to each `answer[i]`.

**Note:** For each query, disregard all edges not in the path between node `ui` and `vi`.

## Examples

**Example 1:**
```
Input: edges = [[1,2]], queries = [[1,1],[1,2]]
Output: [0,1]
Explanation:
Query [1,1]: no edges on path, cost is 0 → 0 valid assignments.
Query [1,2]: one edge → weight 1 gives odd cost → 1 way.
```

**Example 2:**
```
Input: edges = [[1,2],[1,3],[3,4],[3,5]], queries = [[1,4],[3,4],[2,5]]
Output: [2,1,4]
Explanation:
[1,4]: 2 edges → 2 ways. [3,4]: 1 edge → 1 way. [2,5]: 3 edges → 4 ways.
```

## Constraints

- `2 <= n <= 10^5`
- `edges.length == n - 1`
- `edges[i] == [ui, vi]`
- `1 <= queries.length <= 10^5`
- `queries[i] == [ui, vi]`
- `1 <= ui, vi <= n`
- `edges` represents a valid tree.
