/// Maximum total value: pick k subarrays (may overlap/repeat), maximize sum of (max - min).
/// Time: O(n), Space: O(1)
fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let min_val = *nums.iter().min().unwrap() as i64;
    let max_val = *nums.iter().max().unwrap() as i64;

    (max_val - min_val) * k as i64
}

#[derive(Debug)]
struct TestCase {
    nums: Vec<i32>,
    k: i32,
    expected: i64,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            nums: vec![1, 3, 2],
            k: 2,
            expected: 4,
            name: "Example 1",
        },
        TestCase {
            nums: vec![4, 2, 5, 1],
            k: 3,
            expected: 12,
            name: "Example 2",
        },
        TestCase {
            nums: vec![7],
            k: 5,
            expected: 0,
            name: "Single element",
        },
        TestCase {
            nums: vec![1, 1, 1, 1],
            k: 10,
            expected: 0,
            name: "All equal",
        },
        TestCase {
            nums: vec![0, 100],
            k: 1,
            expected: 100,
            name: "Two elements",
        },
    ]
}

fn main() {
    println!("=== Maximum Total Subarray Value I ===\n");

    for tc in test_cases() {
        let result = max_total_value(tc.nums.clone(), tc.k);
        let status = if result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: nums={:?}, k={} -> {} {} (expected {})",
            tc.name, tc.nums, tc.k, result, status, tc.expected
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
                max_total_value(tc.nums, tc.k),
                tc.expected,
                "{}",
                tc.name
            );
        }
    }
}
