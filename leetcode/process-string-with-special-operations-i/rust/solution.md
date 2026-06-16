# Process String with Special Operations I - Rust Solution

## Problem Summary
Simulate four operations on a result string: append a letter, pop the last char (`*`), duplicate (`#`), reverse (`%`).

## Solution: Direct Simulation

```rust
fn process_str(s: String) -> String {
    let mut result = String::new();

    for l in s.chars() {
        match l {
            '*' => { result.pop(); }
            '#' => {
                let copy = result.clone();
                result.push_str(&copy);
            }
            '%' => { result = result.chars().rev().collect(); }
            _ => { result.push(l); }
        }
    }

    result
}
```

### Steps
1. Iterate each character with a `match`
2. `*` — `pop()` removes the last char (no-op if empty)
3. `#` — clone and append to double the string
4. `%` — collect reversed chars into a new `String`
5. Otherwise — push the letter

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n · 2^k), k = number of `#` ops |
| Space | O(n · 2^k) for the result buffer |

The constraints (`s.length <= 20`) bound both n and k, so the result never exceeds 20 · 2^20 chars in the absolute worst case — well within limits.