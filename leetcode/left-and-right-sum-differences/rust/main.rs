/// Left and Right Sum Differences
/// Time: O(n), Space: O(1) extra (excluding output)
fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let total_sum: i32 = nums.iter().sum();

    let mut left_sum = 0;
    let mut answer = Vec::with_capacity(nums.len());

    for &num in &nums {
        let right_sum = total_sum - left_sum - num;
        answer.push((left_sum - right_sum).abs());
        left_sum += num;
    }

    answer
}

#[derive(Debug)]
struct TestCase {
    nums: Vec<i32>,
    expected: Vec<i32>,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            nums: vec![10, 4, 8, 3],
            expected: vec![15, 1, 11, 22],
            name: "Example 1",
        },
        TestCase {
            nums: vec![1],
            expected: vec![0],
            name: "Example 2",
        },
        TestCase {
            nums: vec![1, 2, 3, 4, 5],
            expected: vec![14, 11, 6, 1, 10],
            name: "Increasing sequence",
        },
        TestCase {
            nums: vec![5, 5, 5],
            expected: vec![10, 0, 10],
            name: "Equal elements",
        },
    ]
}

fn main() {
    println!("=== Left and Right Sum Differences ===\n");

    for tc in test_cases() {
        let result = left_right_difference(tc.nums.clone());
        let status = if result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: nums={:?} -> {:?} {} (expected {:?})",
            tc.name, tc.nums, result, status, tc.expected
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
                left_right_difference(tc.nums),
                tc.expected,
                "{}",
                tc.name
            );
        }
    }
}
