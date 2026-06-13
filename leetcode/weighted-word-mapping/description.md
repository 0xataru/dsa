# Weighted Word Mapping

You are given an array of strings `words`, where each string represents a word containing lowercase English letters.

You are also given an integer array `weights` of length 26, where `weights[i]` represents the weight of the `i`th lowercase English letter.

The weight of a word is defined as the sum of the weights of its characters.

For each word, take its weight modulo 26 and map the result to a lowercase English letter using reverse alphabetical order (`0 -> 'z'`, `1 -> 'y'`, ..., `25 -> 'a'`).

Return a string formed by concatenating the mapped characters for all words in order.

## Examples

**Example 1:**
```
Input: words = ["abcd","def","xyz"], weights = [5,3,12,14,1,2,3,2,10,6,6,9,7,8,7,10,8,9,6,9,9,8,3,7,7,2]
Output: "rij"
Explanation:
"abcd" → 34 % 26 = 8 → 'r'
"def"  → 17 % 26 = 17 → 'i'
"xyz"  → 16 % 26 = 16 → 'j'
```

**Example 2:**
```
Input: words = ["a","b","c"], weights = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
Output: "yyy"
Explanation:
Each word has weight 1 → 1 % 26 = 1 → 'y'.
```

**Example 3:**
```
Input: words = ["abcd"], weights = [7,5,3,4,3,5,4,9,4,2,2,7,10,2,5,10,6,1,2,2,4,1,3,4,4,5]
Output: "g"
Explanation:
"abcd" → 19 % 26 = 19 → 'g'.
```

## Constraints

- `1 <= words.length <= 100`
- `1 <= words[i].length <= 10`
- `weights.length == 26`
- `1 <= weights[i] <= 100`
- `words[i]` consists of lowercase English letters.
