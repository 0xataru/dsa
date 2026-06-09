# Maximum Total Subarray Value I

You are given an integer array `nums` of length `n` and an integer `k`.

You need to choose exactly `k` non-empty subarrays `nums[l..r]` of `nums`. Subarrays may overlap, and the exact same subarray (same `l` and `r`) can be chosen more than once.

The value of a subarray `nums[l..r]` is defined as: `max(nums[l..r]) - min(nums[l..r])`.

The total value is the sum of the values of all chosen subarrays.

Return the maximum possible total value you can achieve.

## Examples

**Example 1:**
```
Input: nums = [1,3,2], k = 2
Output: 4
Explanation:
Choose nums[0..1] = [1, 3]: value = 3 - 1 = 2.
Choose nums[0..2] = [1, 3, 2]: value = 3 - 1 = 2.
Total: 2 + 2 = 4.
```

**Example 2:**
```
Input: nums = [4,2,5,1], k = 3
Output: 12
Explanation:
Choose nums[0..3] three times (or mix overlapping subarrays): each gives 5 - 1 = 4.
Total: 4 + 4 + 4 = 12.
```

## Constraints

- `1 <= n == nums.length <= 5 * 10^4`
- `0 <= nums[i] <= 10^9`
- `1 <= k <= 10^5`
