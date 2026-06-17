# Process String with Special Operations II

You are given a string `s` consisting of lowercase English letters and the special characters: `*`, `#`, and `%`.

You are also given an integer `k`.

Build a new string `result` by processing `s` according to the following rules from left to right:

- If the letter is a lowercase English letter append it to `result`.
- A `'*'` removes the last character from `result`, if it exists.
- A `'#'` duplicates the current `result` and appends it to itself.
- A `'%'` reverses the current `result`.

Return the `kth` character of the final string `result`. If `k` is out of the bounds of `result`, return `'.'`.

## Examples

**Example 1:**
```
Input: s = "a#b%*", k = 1
Output: "a"
Explanation:
i=0 'a' → append    → "a"
i=1 '#' → duplicate → "aa"
i=2 'b' → append    → "aab"
i=3 '%' → reverse   → "baa"
i=4 '*' → pop       → "ba"
The final result is "ba". The character at index k = 1 is 'a'.
```

**Example 2:**
```
Input: s = "cd%#*#", k = 3
Output: "d"
Explanation:
i=0 'c' → append    → "c"
i=1 'd' → append    → "cd"
i=2 '%' → reverse   → "dc"
i=3 '#' → duplicate → "dcdc"
i=4 '*' → pop       → "dcd"
i=5 '#' → duplicate → "dcddcd"
The final result is "dcddcd". The character at index k = 3 is 'd'.
```

**Example 3:**
```
Input: s = "z*#", k = 0
Output: "."
Explanation:
i=0 'z' → append    → "z"
i=1 '*' → pop       → ""
i=2 '#' → duplicate → ""
The final result is "". Since index k = 0 is out of bounds, the output is '.'.
```

## Constraints

- `1 <= s.length <= 10^5`
- `s` consists of only lowercase English letters and special characters `*`, `#`, and `%`.
- `0 <= k <= 10^15`
- The length of `result` after processing `s` will not exceed `10^15`.
