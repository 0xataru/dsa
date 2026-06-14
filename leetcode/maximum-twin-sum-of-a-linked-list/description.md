# Maximum Twin Sum of a Linked List

In a linked list of size `n`, where `n` is even, the `i`th node (0-indexed) of the linked list is known as the **twin** of the `(n-1-i)`th node, if `0 <= i <= (n / 2) - 1`.

For example, if `n = 4`, then node 0 is the twin of node 3, and node 1 is the twin of node 2.

The **twin sum** is defined as the sum of a node and its twin.

Given the head of a linked list with even length, return the maximum twin sum of the linked list.

## Examples

**Example 1:**
```
Input: head = [5,4,2,1]
Output: 6
Explanation: twins (0,3) and (1,2) both give 5+1 = 4+2 = 6.
```

**Example 2:**
```
Input: head = [4,2,2,3]
Output: 7
Explanation: 4+3 = 7, 2+2 = 4 → max is 7.
```

**Example 3:**
```
Input: head = [1,100000]
Output: 100001
```

## Constraints

- The number of nodes in the list is an even integer in the range `[2, 10^5]`.
- `1 <= Node.val <= 10^5`
