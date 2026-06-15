# Delete the Middle Node of a Linked List - Rust Solution

## Problem Summary
Delete the node at index `⌊n / 2⌋` and return the new head.

## Solution: Count Length

```rust
fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut len = 0;
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        len += 1;
        cur = node.next.as_ref();
    }

    if len == 1 {
        return None;
    }

    let mid = len / 2;
    let mut cur = head.as_mut();
    for _ in 0..mid - 1 {
        cur = cur.unwrap().next.as_mut();
    }

    let prev = cur.unwrap();
    let next_after_mid = prev.next.as_mut().unwrap().next.take();
    prev.next = next_after_mid;

    head
}
```

### Steps
1. Count nodes → `len`
2. If `len == 1`, return `None`
3. Walk to the node **before** middle (`mid - 1` steps)
4. Skip middle: `prev.next = prev.next.next`

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(1) |

## Why Not Slow/Fast in Rust?

With `Option<Box<ListNode>>`, Rust does not allow two mutable references into the same list at once. Slow/fast works naturally in Go (see `go/main.go`) without `unsafe`.

For Rust on LeetCode, the count-length approach is the standard safe solution — simple and correct.
