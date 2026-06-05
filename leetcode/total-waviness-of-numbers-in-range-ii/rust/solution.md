# Total Waviness of Numbers in Range II - Rust Solution

## Why Brute Force Fails

`num2 <= 10^15` — the range can contain up to 10¹⁵ numbers. A simple loop times out.

## Solution: Digit DP + Prefix Sum

```
total_waviness(num1, num2) = waviness_up_to(num2) - waviness_up_to(num1 - 1)
```

Split `waviness_up_to(n)` into:
1. All numbers with **3 .. d-1 digits** (unbounded DP per length)
2. All **d-digit** numbers from `10^(d-1)` to `n` (bounded DP)

Fixed length avoids leading-zero bugs from variable-length DP.

## Critical DP Detail

When placing digit `d` at `pos`, the peak/valley check on `prev` applies to **every** number in the subtree — not once.

Wrong:
```rust
total += add + dp(child);  // counts `add` once per branch
```

Correct:
```rust
let (sub_count, sub_wav) = dp(child);
total_count += sub_count;
total_waviness += add * sub_count + sub_wav;
```

## State

`(pos, tight, prev, prev_prev)` → `(count, waviness_sum)`

When `pos >= 2`, before recursing:
- Peak: `prev > prev_prev && prev > d`
- Valley: `prev < prev_prev && prev < d`

## Complexity

| Metric | Value |
|--------|-------|
| Time | O(log n × states) per prefix query |
| Space | O(log n × states) |

Two prefix queries handle `10^15` easily.

## Common Bug

When reusing one memo map for `dp_all_len` across several lengths, include `len` in the cache key:

```rust
let key = (len, pos, prev, prev_prev);  // not just (pos, prev, prev_prev)
```

Without `len`, results for 3-digit and 4-digit DP collide and prefix subtraction can go negative.
