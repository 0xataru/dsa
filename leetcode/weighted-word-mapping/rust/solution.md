# Weighted Word Mapping - Rust Solution

## Problem Summary
For each word, sum character weights, take `mod 26`, map to a letter in reverse alphabetical order (`0 → 'z'`, `25 → 'a'`). Concatenate results.

## Solution 1: Explicit Loop

```rust
fn map_word_weights_loop(words: Vec<String>, weights: Vec<i32>) -> String {
    let mut result = String::new();

    for word in words {
        let mut sum = 0;
        for ch in word.bytes() {
            sum += weights[(ch - b'a') as usize];
        }
        result.push((b'z' - (sum % 26) as u8) as char);
    }

    result
}
```

## Solution 2: Iterator Style

```rust
fn map_word_weights_iter(words: Vec<String>, weights: Vec<i32>) -> String {
    words
        .into_iter()
        .map(|word| {
            let sum: i32 = word.bytes().map(|c| weights[(c - b'a') as usize]).sum();
            (b'z' - (sum % 26) as u8) as char
        })
        .collect()
}
```

Both are equivalent — pick based on readability preference.

### Mapping Formula

```
mapped_char = char::from(b'z' - (sum % 26) as u8)
```

| remainder | letter |
|-----------|--------|
| 0 | z |
| 1 | y |
| 8 | r |
| 25 | a |

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(total characters) |
| Space | O(1) extra (output excluded) |

With `words.length <= 100` and `words[i].length <= 10`, this is trivially fast.
