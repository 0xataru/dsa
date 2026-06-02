# Earliest Finish Time for Land and Water Rides I - Go Solution

## Problem Summary
Choose exactly one land ride and one water ride, visit them in either order, and return the earliest possible finish time.

---

## Solution 1: Brute Force Pair Enumeration

### Key Insight
For each pair `(land_i, water_j)` there are only two valid schedules:
1. **Land first**: finish at `max(landFinish, waterStart) + waterDuration`
2. **Water first**: finish at `max(waterFinish, landStart) + landDuration`

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n × m) |
| Space | O(1) |

Simple and sufficient for `n, m <= 100`.

---

## Solution 2: Sorting + Prefix/Suffix Minima (Optimized)

### Key Insight
After fixing the first ride, the second ride is chosen independently of *which* first ride was picked — only its finish time matters.

If the first ride finishes at time `T`, the best second ride among candidates with `start <= T` is:

```
T + min(duration)   // shortest wait after T
```

Among candidates with `start > T`:

```
min(start + duration)   // already open later, pick earliest total finish
```

So for each category we sort rides by `start`, precompute:
- **prefix min** of `duration`
- **suffix min** of `start + duration`

Then for each first ride we binary-search the split point `start <= T` vs `start > T` and query both cases in **O(log m)**.

### Algorithm
1. Build and sort land/water rides by start time
2. Preprocess prefix/suffix arrays for both categories
3. For each land ride: query best water follow-up after `land.finish`
4. For each water ride: query best land follow-up after `water.finish`
5. Return the minimum

### Implementation Sketch

```go
func bestFollowUp(firstFinish int, rides []ride, prefix, suffix []int) int {
    idx := sort.Search(len(rides), func(i int) bool {
        return rides[i].start > firstFinish
    })

    ans := math.MaxInt
    if idx > 0 {
        ans = min(ans, firstFinish+prefix[idx-1])
    }
    if idx < len(rides) {
        ans = min(ans, suffix[idx])
    }
    return ans
}
```

### Complexity

| Metric | Brute Force | Optimized |
|--------|-------------|-----------|
| Time | O(n × m) | O((n + m) log(n + m)) |
| Space | O(1) | O(n + m) |

### When to Prefer Optimized
- Large inputs where `n × m` dominates
- Same logic scales better if constraints grow beyond 100
- Demonstrates the "reduce inner loop to a query" pattern common in scheduling problems

### Comparison

| Approach | Pros | Cons |
|----------|------|------|
| Brute force | Minimal code, no extra memory | O(n × m) |
| Sort + binary search | Sub-quadratic, reusable query | More code, O(n + m) memory |

Both implementations are included in `main.go` and verified against the same test cases.
