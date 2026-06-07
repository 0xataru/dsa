use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Build a binary tree from parent-child descriptions.
/// Time: O(n), Space: O(n)
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

/// Level-order serialization in LeetCode format (null for missing children).
fn tree_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut result = Vec::new();
    let Some(root) = root else {
        return result;
    };

    let mut queue = vec![Some(root)];
    while !queue.is_empty() {
        match queue.remove(0) {
            None => result.push("null".to_string()),
            Some(node) => {
                let node = node.borrow();
                result.push(node.val.to_string());
                queue.push(node.left.clone());
                queue.push(node.right.clone());
            }
        }
    }

    while result.last().is_some_and(|v| v == "null") {
        result.pop();
    }

    result
}

#[derive(Debug)]
struct TestCase {
    descriptions: Vec<Vec<i32>>,
    expected: Vec<&'static str>,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            descriptions: vec![
                vec![20, 15, 1],
                vec![20, 17, 0],
                vec![50, 20, 1],
                vec![50, 80, 0],
                vec![80, 19, 1],
            ],
            expected: vec!["50", "20", "80", "15", "17", "19"],
            name: "Example 1",
        },
        TestCase {
            descriptions: vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]],
            expected: vec!["1", "2", "null", "null", "3", "4"],
            name: "Example 2",
        },
        TestCase {
            descriptions: vec![vec![10, 5, 1], vec![10, 15, 0]],
            expected: vec!["10", "5", "15"],
            name: "Simple two children",
        },
    ]
}

fn main() {
    println!("=== Create Binary Tree From Descriptions ===\n");

    for tc in test_cases() {
        let root = create_binary_tree(tc.descriptions.clone());
        let result = tree_to_vec(root);
        let expected: Vec<String> = tc.expected.iter().map(|s| s.to_string()).collect();
        let status = if result == expected { "✓" } else { "✗" };

        println!(
            "{}: descriptions={:?} -> {:?} {} (expected {:?})",
            tc.name, tc.descriptions, result, status, expected
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_match() {
        for tc in test_cases() {
            let root = create_binary_tree(tc.descriptions);
            let result = tree_to_vec(root);
            let expected: Vec<String> = tc.expected.iter().map(|s| s.to_string()).collect();
            assert_eq!(result, expected, "{}", tc.name);
        }
    }

    #[test]
    fn root_has_no_parent() {
        let descriptions = vec![vec![5, 3, 1], vec![5, 7, 0]];
        let root = create_binary_tree(descriptions).expect("root should exist");
        assert_eq!(root.borrow().val, 5);
        assert_eq!(root.borrow().left.as_ref().unwrap().borrow().val, 3);
        assert_eq!(root.borrow().right.as_ref().unwrap().borrow().val, 7);
    }
}
