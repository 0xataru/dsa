# Process String with Special Operations I

You are given a string `s` consisting of lowercase English letters and the special characters: `*`, `#`, and `%`.

Build a new string `result` by processing `s` according to the following rules from left to right:

- If the letter is a lowercase English letter append it to `result`.
- A `'*'` removes the last character from `result`, if it exists.
- A `'#'` duplicates the current `result` and appends it to itself.
- A `'%'` reverses the current `result`.

Return the final string `result` after processing all characters in `s`.

## Examples

**Example 1:**
```
Input: s = "a#b%*"
Output: "ba"
Explanation:
i=0 'a' → append    → "a"
i=1 '#' → duplicate → "aa"
i=2 'b' → append    → "aab"
i=3 '%' → reverse   → "baa"
i=4 '*' → pop       → "ba"
```

**Example 2:**
```
Input: s = "z*#"
Output: ""
Explanation:
i=0 'z' → append    → "z"
i=1 '*' → pop       → ""
i=2 '#' → duplicate → ""
```

## Constraints

- `1 <= s.length <= 20`
- `s` consists of only lowercase English letters and special characters `*`, `#`, and `%`.