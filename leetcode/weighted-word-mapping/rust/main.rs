/// Map each word's total weight mod 26 to a letter (0 -> 'z', 25 -> 'a').
/// Time: O(total chars), Space: O(1) extra
fn map_word_weights_loop(words: Vec<String>, weights: Vec<i32>) -> String {
    let mut result = String::new();

    for word in words {
        let mut sum = 0;

        for ch in word.bytes() {
            let idx = (ch - b'a') as usize;
            sum += weights[idx];
        }

        let rem = (sum % 26) as u8;
        result.push((b'z' - rem) as char);
    }

    result
}

/// Functional style — same logic.
fn map_word_weights_iter(words: Vec<String>, weights: Vec<i32>) -> String {
    words
        .into_iter()
        .map(|word| {
            let sum: i32 = word
                .bytes()
                .map(|c| weights[(c - b'a') as usize])
                .sum();

            (b'z' - (sum % 26) as u8) as char
        })
        .collect()
}

#[derive(Debug)]
struct TestCase {
    words: Vec<&'static str>,
    weights: Vec<i32>,
    expected: &'static str,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            words: vec!["abcd", "def", "xyz"],
            weights: vec![
                5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7,
                2,
            ],
            expected: "rij",
            name: "Example 1",
        },
        TestCase {
            words: vec!["a", "b", "c"],
            weights: vec![1; 26],
            expected: "yyy",
            name: "Example 2",
        },
        TestCase {
            words: vec!["abcd"],
            weights: vec![
                7, 5, 3, 4, 3, 5, 4, 9, 4, 2, 2, 7, 10, 2, 5, 10, 6, 1, 2, 2, 4, 1, 3, 4, 4, 5,
            ],
            expected: "g",
            name: "Example 3",
        },
    ]
}

fn to_strings(words: Vec<&str>) -> Vec<String> {
    words.into_iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!("=== Weighted Word Mapping ===\n");

    for tc in test_cases() {
        let words = to_strings(tc.words.clone());
        let loop_result = map_word_weights_loop(words.clone(), tc.weights.clone());
        let iter_result = map_word_weights_iter(words, tc.weights.clone());

        let loop_ok = if loop_result == tc.expected { "✓" } else { "✗" };
        let iter_ok = if iter_result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: words={:?} -> loop={:?} {}, iter={:?} {} (expected {:?})",
            tc.name, tc.words, loop_result, loop_ok, iter_result, iter_ok, tc.expected
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_both(tc: &TestCase) {
        let words = to_strings(tc.words.clone());
        let loop_result = map_word_weights_loop(words.clone(), tc.weights.clone());
        let iter_result = map_word_weights_iter(words, tc.weights.clone());

        assert_eq!(loop_result, tc.expected, "{} (loop)", tc.name);
        assert_eq!(iter_result, tc.expected, "{} (iter)", tc.name);
    }

    #[test]
    fn examples_match() {
        for tc in test_cases() {
            run_both(&tc);
        }
    }
}
