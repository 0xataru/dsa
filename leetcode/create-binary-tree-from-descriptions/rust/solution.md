# Create Binary Tree From Descriptions - Rust Solution

## Problem Summary
Given edges `[parent, child, isLeft]`, build the binary tree and return the root.

## Solution Approach: HashMap + HashSet

### Algorithm
1. Create a `HashMap<value, Rc<RefCell<TreeNode>>>` for all nodes
2. For each description:
   - Lazily create parent and child nodes
   - Attach child to parent's left (`isLeft == 1`) or right (`isLeft == 0`)
   - Mark `child` in a `HashSet` of all children
3. The root is the parent value that never appears as a child

### Implementation

```rust
fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut nodes: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
    let mut children: HashSet<i32> = HashSet::new();

    for desc in &descriptions {
        let parent = desc[0];
        let child = desc[1];
        let is_left = desc[2];

        let parent_node = nodes
            .entry(parent)
            .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent))))
            .clone();

        let child_node = nodes
            .entry(child)
            .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child))))
            .clone();

        if is_left == 1 {
            parent_node.borrow_mut().left = Some(child_node);
        } else {
            parent_node.borrow_mut().right = Some(child_node);
        }

        children.insert(child);
    }

    for desc in descriptions {
        let parent = desc[0];
        if !children.contains(&parent) {
            return nodes.get(&parent).cloned();
        }
    }

    None
}
```

### Why `Rc<RefCell<>>`?
LeetCode's Rust tree API uses shared ownership with interior mutability so multiple references can link parent ↔ child.

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(n) |

Where `n = descriptions.length`.
