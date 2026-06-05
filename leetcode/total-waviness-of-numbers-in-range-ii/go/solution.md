# Total Waviness of Numbers in Range II - Go Solution

## Why Brute Force Fails

`num2 <= 10^15` — the range can contain up to 10¹⁵ numbers. A simple loop times out.

## Solution: Digit DP + Prefix Sum

```
totalWaviness(num1, num2) = wavinessUpTo(num2) - wavinessUpTo(num1 - 1)
```

Split `wavinessUpTo(n)` into:
1. All numbers with **3 .. d-1 digits** (unbounded DP per length)
2. All **d-digit** numbers from `10^(d-1)` to `n` (bounded DP)

Fixed length avoids leading-zero bugs from variable-length DP.

## Critical DP Detail

When placing digit `d` at `pos`, the peak/valley check on `prev` applies to **every** number in the subtree — not once.

Wrong:
```go
total += add + dp(child)  // counts add once per branch
```

Correct:
```go
subCount, subWav := dp(child)
totalCount += subCount
totalWaviness += add*subCount + subWav
```

## State

`(pos, tight, prev, prevPrev)` → `(count, wavinessSum)`

When `pos >= 2`, before recursing:
- Peak: `prev > prevPrev && prev > d`
- Valley: `prev < prevPrev && prev < d`

## Complexity

| Metric | Value |
|--------|-------|
| Time | O(log n × states) per prefix query |
| Space | O(log n × states) |

Two prefix queries handle `10^15` easily.

## Common Bug

When reusing one memo map for `dpAllLen` across several lengths, include `length` in the cache key:

```go
key := allLenKey{length, pos, prev, prevPrev}
```

Without `length`, results for 3-digit and 4-digit DP collide and prefix subtraction can go negative.
