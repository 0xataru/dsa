# Create Binary Tree From Descriptions - Go Solution

## Problem Summary
Given edges `[parent, child, isLeft]`, build the binary tree and return the root.

## Solution Approach: HashMap + Set

### Algorithm
1. Keep `map[int]*TreeNode` for all nodes (create on demand)
2. For each description:
   - Link child to parent's left (`isLeft == 1`) or right (`isLeft == 0`)
   - Mark child in a set of all children
3. The root is the parent value that never appears as a child

### Implementation

```go
func createBinaryTree(descriptions [][]int) *TreeNode {
    nodes := make(map[int]*TreeNode)
    children := make(map[int]bool)

    getNode := func(val int) *TreeNode {
        if node, ok := nodes[val]; ok {
            return node
        }
        node := &TreeNode{Val: val}
        nodes[val] = node
        return node
    }

    for _, desc := range descriptions {
        parent := getNode(desc[0])
        child := getNode(desc[1])

        if desc[2] == 1 {
            parent.Left = child
        } else {
            parent.Right = child
        }

        children[desc[1]] = true
    }

    for _, desc := range descriptions {
        if !children[desc[0]] {
            return nodes[desc[0]]
        }
    }

    return nil
}
```

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(n) |

Where `n = len(descriptions)`.
