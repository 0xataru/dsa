#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn vec_to_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in values.iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }));
    }
    head
}

fn list_to_vec(head: Option<&ListNode>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut curr = head;

    while let Some(node) = curr {
        result.push(node.val);
        curr = node.next.as_deref();
    }

    result
}

/// Count length, delete node at index n/2.
/// Time: O(n), Space: O(1)
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

#[derive(Debug)]
struct TestCase {
    values: Vec<i32>,
    expected: Vec<i32>,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            values: vec![1, 3, 4, 7, 1, 2, 6],
            expected: vec![1, 3, 4, 1, 2, 6],
            name: "Example 1",
        },
        TestCase {
            values: vec![1, 2, 3, 4],
            expected: vec![1, 2, 4],
            name: "Example 2",
        },
        TestCase {
            values: vec![2, 1],
            expected: vec![2],
            name: "Example 3",
        },
        TestCase {
            values: vec![1],
            expected: vec![],
            name: "Single node",
        },
    ]
}

fn main() {
    println!("=== Delete the Middle Node of a Linked List ===\n");

    for tc in test_cases() {
        let result = list_to_vec(delete_middle(vec_to_list(&tc.values)).as_deref());
        let status = if result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: {:?} -> {:?} {} (expected {:?})",
            tc.name, tc.values, result, status, tc.expected
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_match() {
        for tc in test_cases() {
            assert_eq!(
                list_to_vec(delete_middle(vec_to_list(&tc.values)).as_deref()),
                tc.expected,
                "{}",
                tc.name
            );
        }
    }
}
