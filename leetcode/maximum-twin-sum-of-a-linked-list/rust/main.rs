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

/// Copy values into a vector, then pair from both ends.
/// Time: O(n), Space: O(n)
fn pair_sum_vec(head: Option<Box<ListNode>>) -> i32 {
    let mut nums = Vec::new();
    let mut curr = head.as_ref();

    while let Some(node) = curr {
        nums.push(node.val);
        curr = node.next.as_ref();
    }

    let mut ans = 0;
    let n = nums.len();

    for i in 0..n / 2 {
        ans = ans.max(nums[i] + nums[n - 1 - i]);
    }

    ans
}

fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;

    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}

/// Split at middle, reverse second half, compare pairs in-place.
/// Time: O(n), Space: O(1) extra (list is mutated)
fn pair_sum_inplace(head: Option<Box<ListNode>>) -> i32 {
    let mut len = 0;
    let mut cur = head.as_ref();

    while let Some(node) = cur {
        len += 1;
        cur = node.next.as_ref();
    }

    let mut head = head;
    let mut mid = &mut head;

    for _ in 0..len / 2 {
        mid = &mut mid.as_mut().unwrap().next;
    }

    let second = mid.take();
    let second = reverse_list(second);

    let mut p1 = head.as_ref();
    let mut p2 = second.as_ref();
    let mut ans = 0;

    while let (Some(a), Some(b)) = (p1, p2) {
        ans = ans.max(a.val + b.val);
        p1 = a.next.as_ref();
        p2 = b.next.as_ref();
    }

    ans
}

#[derive(Debug)]
struct TestCase {
    values: Vec<i32>,
    expected: i32,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            values: vec![5, 4, 2, 1],
            expected: 6,
            name: "Example 1",
        },
        TestCase {
            values: vec![4, 2, 2, 3],
            expected: 7,
            name: "Example 2",
        },
        TestCase {
            values: vec![1, 100_000],
            expected: 100_001,
            name: "Example 3",
        },
    ]
}

fn main() {
    println!("=== Maximum Twin Sum of a Linked List ===\n");

    for tc in test_cases() {
        let head_vec = vec_to_list(&tc.values);
        let head_inplace = vec_to_list(&tc.values);

        let vec_result = pair_sum_vec(head_vec);
        let inplace_result = pair_sum_inplace(head_inplace);

        let vec_ok = if vec_result == tc.expected { "✓" } else { "✗" };
        let inplace_ok = if inplace_result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: {:?} -> vec={}, inplace={} (expected {}) {} / {}",
            tc.name,
            tc.values,
            vec_result,
            inplace_result,
            tc.expected,
            vec_ok,
            inplace_ok
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_both(tc: &TestCase) {
        let vec_result = pair_sum_vec(vec_to_list(&tc.values));
        let inplace_result = pair_sum_inplace(vec_to_list(&tc.values));

        assert_eq!(vec_result, tc.expected, "{} (vec)", tc.name);
        assert_eq!(inplace_result, tc.expected, "{} (inplace)", tc.name);
    }

    #[test]
    fn examples_match() {
        for tc in test_cases() {
            run_both(&tc);
        }
    }
}
