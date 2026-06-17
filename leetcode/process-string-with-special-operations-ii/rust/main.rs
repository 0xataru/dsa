/// Find the kth character of the final string without ever materializing it.
///
/// The result can grow up to 10^15 characters, so direct simulation is
/// impossible. Instead we compute the length after each prefix (capped at the
/// stated bound to avoid overflow) in a forward pass, then walk the operations
/// backwards, rewriting the target index `k` until it lands on the letter that
/// originally produced it.
///
/// Time: O(n) for both passes.
/// Space: O(n) for the prefix-length array.
fn process_str(s: String, k: i64) -> char {
    let k0 = k as u64;
    // One past the maximum possible result length; lengths are clamped here so
    // doubling never overflows while staying large enough for any valid `k`.
    let limit = 1_000_000_000_000_000u64 + 1;

    let bytes = s.as_bytes();
    let n = bytes.len();

    // len[i] = length of result after processing the first i characters.
    let mut len = vec![0u64; n + 1];
    for i in 0..n {
        len[i + 1] = match bytes[i] {
            b'a'..=b'z' => (len[i] + 1).min(limit),
            b'*' => len[i].saturating_sub(1),
            b'#' => len[i].saturating_mul(2).min(limit),
            b'%' => len[i],
            _ => unreachable!(),
        };
    }

    let mut cur_len = len[n];
    if k0 >= cur_len {
        return '.';
    }

    // Walk backwards, translating `k` into the index it occupied before each op.
    let mut k = k0;
    for i in (0..n).rev() {
        match bytes[i] {
            b'a'..=b'z' => {
                let prev_len = len[i];
                // The appended letter sits at the last position; if `k` points
                // there, this is the source character.
                if k == prev_len {
                    return bytes[i] as char;
                }
                cur_len = prev_len;
            }
            b'*' => {
                // A removed character only made the string shorter; nothing of
                // it survives, so `k` is unaffected and the length grows by one.
                cur_len += 1;
            }
            b'#' => {
                let prev_len = len[i];
                // Duplication: the second half mirrors the first, so fold `k`
                // back into the original copy.
                if prev_len > 0 {
                    k %= prev_len;
                }
                cur_len = prev_len;
            }
            b'%' => {
                // Reversal maps index `k` to `cur_len - 1 - k`.
                if cur_len > 0 {
                    k = cur_len - 1 - k;
                }
            }
            _ => unreachable!(),
        }
    }

    '.'
}

#[derive(Debug)]
struct TestCase {
    s: &'static str,
    k: i64,
    expected: char,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            s: "a#b%*",
            k: 1,
            expected: 'a',
            name: "Example 1",
        },
        TestCase {
            s: "cd%#*#",
            k: 3,
            expected: 'd',
            name: "Example 2",
        },
        TestCase {
            s: "z*#",
            k: 0,
            expected: '.',
            name: "Example 3 (out of bounds)",
        },
        TestCase {
            s: "abc",
            k: 0,
            expected: 'a',
            name: "No special chars, first",
        },
        TestCase {
            s: "abc",
            k: 2,
            expected: 'c',
            name: "No special chars, last",
        },
        TestCase {
            s: "ab#%",
            k: 0,
            expected: 'b',
            name: "Duplicate then reverse",
        },
        TestCase {
            s: "ab#%",
            k: 3,
            expected: 'a',
            name: "Duplicate then reverse, last",
        },
        TestCase {
            s: "a%",
            k: 0,
            expected: 'a',
            name: "Reverse single char",
        },
        TestCase {
            s: "ab###",
            k: 15,
            expected: 'b',
            name: "Repeated duplication",
        },
    ]
}

fn main() {
    println!("=== Process String with Special Operations II ===\n");

    for tc in test_cases() {
        let result = process_str(tc.s.to_string(), tc.k);
        let status = if result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: {:?}, k={} -> {:?} {} (expected {:?})",
            tc.name, tc.s, tc.k, result, status, tc.expected
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
                process_str(tc.s.to_string(), tc.k),
                tc.expected,
                "{}",
                tc.name
            );
        }
    }
}
