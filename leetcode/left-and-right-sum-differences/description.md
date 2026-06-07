# Left and Right Sum Differences

You are given a 0-indexed integer array `nums` of size `n`.

Define two arrays `leftSum` and `rightSum` where:

- `leftSum[i]` is the sum of elements to the left of the index `i` in the array `nums`. If there is no such element, `leftSum[i] = 0`.
- `rightSum[i]` is the sum of elements to the right of the index `i` in the array `nums`. If there is no such element, `rightSum[i] = 0`.

Return an integer array `answer` of size `n` where `answer[i] = |leftSum[i] - rightSum[i]|`.

## Examples

**Example 1:**
```
Input: nums = [10,4,8,3]
Output: [15,1,11,22]
Explanation: leftSum = [0,10,14,22], rightSum = [15,11,3,0].
answer = [|0-15|, |10-11|, |14-3|, |22-0|] = [15,1,11,22].
```

**Example 2:**
```
Input: nums = [1]
Output: [0]
Explanation: leftSum = [0], rightSum = [0], answer = [0].
```

## Constraints

- `1 <= nums.length <= 1000`
- `1 <= nums[i] <= 10^5`
