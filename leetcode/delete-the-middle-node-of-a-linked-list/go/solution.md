# Delete the Middle Node of a Linked List - Go Solution

## Problem Summary
Delete the node at index `⌊n / 2⌋` and return the new head.

## Solution 1: Count Length

1. Count nodes
2. Walk to index `mid - 1`
3. `cur.Next = cur.Next.Next`

Time: O(n), Space: O(1)

## Solution 2: Slow / Fast Pointers

```go
slow, fast := head, head.Next
for fast != nil && fast.Next != nil && fast.Next.Next != nil {
    slow = slow.Next
    fast = fast.Next.Next
}
slow.Next = slow.Next.Next
```

Important: condition is `fast.Next.Next != nil`, not just `fast.Next != nil`.

One pass, `slow` ends at the node before the middle.

## Edge Case

Single node → return `nil`.
