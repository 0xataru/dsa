use std::collections::HashMap;

const NONE: u8 = 10;

/// Brute force — only for small ranges (verification).
fn total_waviness_brute(num1: i64, num2: i64) -> i64 {
    let mut waviness = 0;

    for num in num1..=num2 {
        let digits: Vec<u8> = num
            .to_string()
            .bytes()
            .map(|b| b - b'0')
            .collect();

        if digits.len() < 3 {
            continue;
        }

        for i in 1..digits.len() - 1 {
            let prev = digits[i - 1];
            let curr = digits[i];
            let next = digits[i + 1];

            if (curr > prev && curr > next) || (curr < prev && curr < next) {
                waviness += 1;
            }
        }
    }

    waviness
}

/// Returns (count of numbers, total waviness) for all completions.
fn dp_bounded(
    digits: &[u8],
    pos: usize,
    tight: bool,
    prev: u8,
    prev_prev: u8,
    memo: &mut HashMap<(usize, bool, u8, u8), (i64, i64)>,
) -> (i64, i64) {
    if pos == digits.len() {
        return (1, 0);
    }

    let key = (pos, tight, prev, prev_prev);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let lo = if pos == 0 { 1 } else { 0 };
    let hi = if tight { digits[pos] } else { 9 };

    let mut total_count = 0i64;
    let mut total_waviness = 0i64;

    for d in lo..=hi {
        let mut add = 0i64;
        if pos >= 2 {
            if prev > prev_prev && prev > d {
                add = 1;
            } else if prev < prev_prev && prev < d {
                add = 1;
            }
        }

        let (sub_count, sub_waviness) = dp_bounded(
            digits,
            pos + 1,
            tight && d == hi,
            d,
            if pos >= 1 { prev } else { NONE },
            memo,
        );

        // `add` applies to every number in the subtree.
        total_count += sub_count;
        total_waviness += add * sub_count + sub_waviness;
    }

    let result = (total_count, total_waviness);
    memo.insert(key, result);
    result
}

fn dp_all_len(
    len: usize,
    pos: usize,
    prev: u8,
    prev_prev: u8,
    memo: &mut HashMap<(usize, usize, u8, u8), (i64, i64)>,
) -> (i64, i64) {
    if pos == len {
        return (1, 0);
    }

    // `len` must be part of the key — same (pos, prev, prev_prev) means different
    // subtrees for different target lengths.
    let key = (len, pos, prev, prev_prev);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let lo = if pos == 0 { 1 } else { 0 };

    let mut total_count = 0i64;
    let mut total_waviness = 0i64;

    for d in lo..=9 {
        let mut add = 0i64;
        if pos >= 2 {
            if prev > prev_prev && prev > d {
                add = 1;
            } else if prev < prev_prev && prev < d {
                add = 1;
            }
        }

        let (sub_count, sub_waviness) = dp_all_len(len, pos + 1, d, if pos >= 1 { prev } else { NONE }, memo);

        total_count += sub_count;
        total_waviness += add * sub_count + sub_waviness;
    }

    let result = (total_count, total_waviness);
    memo.insert(key, result);
    result
}

/// Sum of waviness for all numbers in [0, n].
fn waviness_up_to(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    if n < 100 {
        return total_waviness_brute(0, n);
    }

    let digits: Vec<u8> = n
        .to_string()
        .bytes()
        .map(|b| b - b'0')
        .collect();
    let len = digits.len();

    let mut ans = 0i64;
    let mut memo_all = HashMap::new();

    for l in 3..len {
        ans += dp_all_len(l, 0, NONE, NONE, &mut memo_all).1;
    }

    let mut memo_bounded = HashMap::new();
    ans += dp_bounded(&digits, 0, true, NONE, NONE, &mut memo_bounded).1;
    ans
}

/// Total waviness for [num1, num2] via prefix sums.
fn total_waviness(num1: i64, num2: i64) -> i64 {
    waviness_up_to(num2) - waviness_up_to(num1 - 1)
}

#[derive(Debug)]
struct TestCase {
    num1: i64,
    num2: i64,
    expected: i64,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            num1: 120,
            num2: 130,
            expected: 3,
            name: "Example 1",
        },
        TestCase {
            num1: 198,
            num2: 202,
            expected: 3,
            name: "Example 2",
        },
        TestCase {
            num1: 4848,
            num2: 4848,
            expected: 2,
            name: "Example 3",
        },
        TestCase {
            num1: 1,
            num2: 99,
            expected: 0,
            name: "All numbers have fewer than 3 digits",
        },
        TestCase {
            num1: 100,
            num2: 100,
            expected: 0,
            name: "Three digits, no peak or valley",
        },
        TestCase {
            num1: 5720,
            num2: 10850,
            expected: 6429,
            name: "Wrong Answer case (shared memo bug)",
        },
        TestCase {
            num1: 6613514,
            num2: 9155102,
            expected: 0,
            name: "TLE case from Part II",
        },
    ]
}

fn main() {
    println!("=== Digit DP (Part II) ===\n");

    for tc in &test_cases()[0..6] {
        let result = total_waviness(tc.num1, tc.num2);
        let status = if result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: num1={}, num2={} -> expected={}, got={} {}",
            tc.name, tc.num1, tc.num2, tc.expected, result, status
        );
    }

    let tle = &test_cases()[6];
    println!(
        "\n{}: num1={}, num2={} -> {}",
        tle.name, tle.num1, tle.num2,
        total_waviness(tle.num1, tle.num2)
    );

    println!("\n=== Brute vs DP cross-check (small ranges) ===\n");
    for (lo, hi) in [(120, 130), (198, 202), (1010, 1011), (1000, 1500)] {
        let brute = total_waviness_brute(lo, hi);
        let fast = total_waviness(lo, hi);
        let ok = if brute == fast { "✓" } else { "✗" };
        println!("[{lo}, {hi}]: brute={brute}, dp={fast} {ok}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dp_matches_examples() {
        for tc in &test_cases()[0..6] {
            assert_eq!(
                total_waviness(tc.num1, tc.num2),
                tc.expected,
                "{}",
                tc.name
            );
        }
    }

    #[test]
    fn dp_matches_brute_on_small_ranges() {
        let ranges = [
            (120, 130),
            (198, 202),
            (4848, 4848),
            (1, 99),
            (1000, 1200),
            (1010, 1011),
            (5720, 10850),
        ];

        for (lo, hi) in ranges {
            assert_eq!(
                total_waviness(lo, hi),
                total_waviness_brute(lo, hi),
                "range [{lo}, {hi}]"
            );
        }
    }

    #[test]
    fn tle_case_completes_quickly() {
        let result = total_waviness(6613514, 9155102);
        assert!(result > 0);
    }
}
