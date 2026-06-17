# Process String with Special Operations II - Rust Solution

## Problem Summary
Same four operations as version I (append, pop `*`, duplicate `#`, reverse `%`), but the result may reach `10^15` characters. We must return only the `kth` character, so building the string is out of the question.

## Solution: Length Tracking + Backward Walk

The key insight: we never need the string, only where index `k` *came from*. So:

1. **Forward pass** — compute `len[i]`, the length of `result` after the first `i` characters. Clamp every length at `10^15 + 1` so the doubling from `#` cannot overflow `u64` while staying large enough for any valid `k`.
2. **Bounds check** — if `k >= len[n]`, the index does not exist, return `'.'`.
3. **Backward pass** — walk the operations in reverse, rewriting `k` into the index it occupied *before* each operation:
   - letter: it sits at the last slot. If `k == len[i]` (the length before appending), this is the source character — return it.
   - `*`: the removed character left no trace, `k` is unchanged; the length was one larger before.
   - `#`: the second half mirrors the first, so `k %= prev_len` folds it back into the original copy.
   - `%`: a reversal sends index `k` to `cur_len - 1 - k`.

When the loop finishes the walk has resolved which letter produced position `k`.

```rust
fn process_str(s: String, k: i64) -> char {
    let k0 = k as u64;
    let limit = 1_000_000_000_000_000u64 + 1;
    let bytes = s.as_bytes();
    let n = bytes.len();

    let mut len = vec![0u64; n + 1];
    for i in 0..n {
        len[i + 1] = match bytes[i] {
            b'a'..=b'z' => (len[i] + 1).min(limit),
            b'*' => len[i].saturating_sub(1),
            b'#' => len[i].saturating_mul(2).min(limit),
            b'%' => len[i],
            _ => unreachable!(),
        };
    }

    let mut cur_len = len[n];
    if k0 >= cur_len {
        return '.';
    }

    let mut k = k0;
    for i in (0..n).rev() {
        match bytes[i] {
            b'a'..=b'z' => {
                let prev_len = len[i];
                if k == prev_len {
                    return bytes[i] as char;
                }
                cur_len = prev_len;
            }
            b'*' => cur_len += 1,
            b'#' => {
                let prev_len = len[i];
                if prev_len > 0 {
                    k %= prev_len;
                }
                cur_len = prev_len;
            }
            b'%' => {
                if cur_len > 0 {
                    k = cur_len - 1 - k;
                }
            }
            _ => unreachable!(),
        }
    }

    '.'
}
```

### Why the clamp matters
With up to `10^5` characters, a string of `#`s would double the length toward `2^(10^5)`. Clamping at `10^15 + 1` keeps arithmetic in `u64` and is harmless: any clamped length is far above the largest valid `k` (`10^15`), so the bounds check and `% prev_len` reductions still behave correctly.

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) — one forward pass, one backward pass |
| Space | O(n) for the prefix-length array |
