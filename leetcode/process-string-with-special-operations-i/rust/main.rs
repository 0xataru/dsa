/// Simulate each character operation on a mutable string.
/// Time: O(n * 2^k) where k is the number of '#' ops (each can double length).
/// Space: O(n * 2^k) for the result buffer.
fn process_str(s: String) -> String {
    let mut result = String::new();

    for l in s.chars() {
        match l {
            '*' => {
                result.pop();
            }
            '#' => {
                let copy = result.clone();
                result.push_str(&copy);
            }
            '%' => {
                result = result.chars().rev().collect();
            }
            _ => {
                result.push(l);
            }
        }
    }

    result
}

#[derive(Debug)]
struct TestCase {
    s: &'static str,
    expected: &'static str,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            s: "a#b%*",
            expected: "ba",
            name: "Example 1",
        },
        TestCase {
            s: "z*#",
            expected: "",
            name: "Example 2",
        },
        TestCase {
            s: "abc",
            expected: "abc",
            name: "No special chars",
        },
        TestCase {
            s: "***",
            expected: "",
            name: "Pop on empty",
        },
        TestCase {
            s: "a%",
            expected: "a",
            name: "Reverse single char",
        },
        TestCase {
            s: "ab#%",
            expected: "baba",
            name: "Duplicate then reverse",
        },
    ]
}

fn main() {
    println!("=== Process String with Special Operations I ===\n");

    for tc in test_cases() {
        let result = process_str(tc.s.to_string());
        let status = if result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: {:?} -> {:?} {} (expected {:?})",
            tc.name, tc.s, result, status, tc.expected
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
                process_str(tc.s.to_string()),
                tc.expected,
                "{}",
                tc.name
            );
        }
    }
}