# Maximum Twin Sum of a Linked List - Go Solution

## Solution 1: Slice

Collect values, pair `nums[i]` with `nums[n-1-i]`.

- Time: O(n), Space: O(n)

## Solution 2: Split + Reverse

1. Find middle (count length, advance `length/2` steps)
2. Reverse second half
3. Walk both halves in parallel

- Time: O(n), Space: O(1) extra

## Why Vec/Slice Often Beats In-Place on Benchmarks

Same reasons as Rust: one contiguous pass, better cache locality, fewer pointer operations than split + reverse + compare.

In Go, in-place reverse is cheaper than Rust `Box` moves, but the extra passes still add overhead.

## Recommendation

Use the **slice** approach unless the problem explicitly requires O(1) extra space.
