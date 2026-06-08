/// Partition array: less than pivot, then equal, then greater (stable within groups).
/// Time: O(n), Space: O(n)
fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut left = Vec::new();
    let mut equal = Vec::new();
    let mut right = Vec::new();

    for num in nums {
        if num < pivot {
            left.push(num);
        } else if num == pivot {
            equal.push(num);
        } else {
            right.push(num);
        }
    }

    left.extend(equal);
    left.extend(right);
    left
}

#[derive(Debug)]
struct TestCase {
    nums: Vec<i32>,
    pivot: i32,
    expected: Vec<i32>,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            nums: vec![9, 12, 5, 10, 14, 3, 10],
            pivot: 10,
            expected: vec![9, 5, 3, 10, 10, 12, 14],
            name: "Example 1",
        },
        TestCase {
            nums: vec![-3, 4, 3, 2],
            pivot: 2,
            expected: vec![-3, 2, 4, 3],
            name: "Example 2",
        },
        TestCase {
            nums: vec![2, 2, 2],
            pivot: 2,
            expected: vec![2, 2, 2],
            name: "All equal to pivot",
        },
        TestCase {
            nums: vec![1, 2, 3, 4, 5],
            pivot: 3,
            expected: vec![1, 2, 3, 4, 5],
            name: "Already partitioned",
        },
    ]
}

fn main() {
    println!("=== Partition Array According to Given Pivot ===\n");

    for tc in test_cases() {
        let result = pivot_array(tc.nums.clone(), tc.pivot);
        let status = if result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: nums={:?}, pivot={} -> {:?} {} (expected {:?})",
            tc.name, tc.nums, tc.pivot, result, status, tc.expected
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
                pivot_array(tc.nums, tc.pivot),
                tc.expected,
                "{}",
                tc.name
            );
        }
    }
}
