use std::collections::BinaryHeap;

/// Maximum total value with k distinct subarrays.
/// Time: O(n^2 log n) preprocessing + O(k log n) greedy, Space: O(n log n)
fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    if n == 0 || k == 0 {
        return 0;
    }

    let mut lg = vec![0usize; n + 1];
    for i in 2..=n {
        lg[i] = lg[i / 2] + 1;
    }

    let max_log = lg[n] + 1;
    let mut st_max = vec![vec![0i32; max_log]; n];
    let mut st_min = vec![vec![0i32; max_log]; n];

    for i in 0..n {
        st_max[i][0] = nums[i];
        st_min[i][0] = nums[i];
    }

    for j in 1..max_log {
        let len = 1usize << j;
        let half = len >> 1;

        for i in 0..=n.saturating_sub(len) {
            st_max[i][j] = st_max[i][j - 1].max(st_max[i + half][j - 1]);
            st_min[i][j] = st_min[i][j - 1].min(st_min[i + half][j - 1]);
        }
    }

    let query = |l: usize, r: usize| -> i64 {
        let k = lg[r - l + 1];
        let mx = st_max[l][k].max(st_max[r + 1 - (1usize << k)][k]);
        let mn = st_min[l][k].min(st_min[r + 1 - (1usize << k)][k]);
        (mx as i64) - (mn as i64)
    };

    let mut heap: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();

    for l in 0..n {
        let val = query(l, n - 1);
        heap.push((val, l, n - 1));
    }

    let mut ans = 0i64;

    for _ in 0..k {
        let Some((val, l, r)) = heap.pop() else {
            break;
        };

        ans += val;

        if r > l {
            let next_val = query(l, r - 1);
            heap.push((next_val, l, r - 1));
        }
    }

    ans
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
            k: 1,
            expected: 0,
            name: "Single element",
        },
        TestCase {
            nums: vec![0, 100],
            k: 1,
            expected: 100,
            name: "Two elements, best subarray",
        },
    ]
}

fn main() {
    println!("=== Maximum Total Subarray Value II ===\n");

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
