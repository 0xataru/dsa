/// Waviness of a single number: count of peak/valley digits (not first/last).
fn waviness_of_number(num: i32) -> i32 {
    let digits: Vec<u8> = num
        .to_string()
        .bytes()
        .map(|b| b - b'0')
        .collect();

    if digits.len() < 3 {
        return 0;
    }

    let mut count = 0;
    for i in 1..digits.len() - 1 {
        let prev = digits[i - 1];
        let curr = digits[i];
        let next = digits[i + 1];

        if (curr > prev && curr > next) || (curr < prev && curr < next) {
            count += 1;
        }
    }

    count
}

/// Total waviness for all numbers in [num1, num2].
/// Time: O((num2 - num1 + 1) * d), where d is digit count per number
/// Space: O(d) per number for digit buffer
fn total_waviness(num1: i32, num2: i32) -> i32 {
    let mut waviness = 0;

    for num in num1..=num2 {
        waviness += waviness_of_number(num);
    }

    waviness
}

#[derive(Debug)]
struct TestCase {
    num1: i32,
    num2: i32,
    expected: i32,
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
    ]
}

fn main() {
    println!("=== Total Waviness Test Cases ===\n");

    for tc in test_cases() {
        let result = total_waviness(tc.num1, tc.num2);
        let status = if result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: num1={}, num2={} -> expected={}, got={} {}",
            tc.name, tc.num1, tc.num2, tc.expected, result, status
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waviness_of_number_examples() {
        assert_eq!(waviness_of_number(120), 1);
        assert_eq!(waviness_of_number(201), 1);
        assert_eq!(waviness_of_number(4848), 2);
        assert_eq!(waviness_of_number(12), 0);
        assert_eq!(waviness_of_number(100), 0);
    }

    #[test]
    fn total_waviness_matches_examples() {
        for tc in test_cases() {
            assert_eq!(
                total_waviness(tc.num1, tc.num2),
                tc.expected,
                "{}",
                tc.name
            );
        }
    }
}
