# Earliest Finish Time for Land and Water Rides I

You are given two categories of theme park attractions: land rides and water rides.

**Land rides**
- `landStartTime[i]` – the earliest time the ith land ride can be boarded.
- `landDuration[i]` – how long the ith land ride lasts.

**Water rides**
- `waterStartTime[j]` – the earliest time the jth water ride can be boarded.
- `waterDuration[j]` – how long the jth water ride lasts.

A tourist must experience exactly one ride from each category, in either order.

A ride may be started at its opening time or any later moment.
If a ride is started at time `t`, it finishes at time `t + duration`.
Immediately after finishing one ride the tourist may board the other (if it is already open) or wait until it opens.

Return the earliest possible time at which the tourist can finish both rides.

## Examples

**Example 1:**
```
Input: landStartTime = [2,8], landDuration = [4,1], waterStartTime = [6], waterDuration = [3]
Output: 9
Explanation:
Plan A (land ride 0 → water ride 0):
Start land ride 0 at time 2. Finish at 6.
Water ride 0 opens at 6. Start immediately at 6, finish at 9.
Plan B (water ride 0 → land ride 1):
Start water ride 0 at 6. Finish at 9.
Land ride 1 opens at 8. Start at 9, finish at 10.
Plan C (land ride 1 → water ride 0):
Start land ride 1 at 8. Finish at 9.
Water ride 0 opened at 6. Start at 9, finish at 12.
Plan D (water ride 0 → land ride 0):
Start water ride 0 at 6. Finish at 9.
Land ride 0 opened at 2. Start at 9, finish at 13.
Plan A gives the earliest finish time of 9.
```

**Example 2:**
```
Input: landStartTime = [5], landDuration = [3], waterStartTime = [1], waterDuration = [10]
Output: 14
Explanation:
Plan A (water ride 0 → land ride 0):
Start water ride 0 at 1. Finish at 11.
Land ride 0 opened at 5. Start at 11, finish at 14.
Plan B (land ride 0 → water ride 0):
Start land ride 0 at 5. Finish at 8.
Water ride 0 opened at 1. Start at 8, finish at 18.
Plan A provides the earliest finish time of 14.
```

## Constraints

- `1 <= n, m <= 100`
- `landStartTime.length == landDuration.length == n`
- `waterStartTime.length == waterDuration.length == m`
- `1 <= landStartTime[i], landDuration[i], waterStartTime[j], waterDuration[j] <= 1000`
