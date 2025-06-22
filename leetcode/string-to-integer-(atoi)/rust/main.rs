/// String to Integer (atoi) - converts a string to a 32-bit signed integer
/// Time Complexity: O(n) - where n is the length of the string
/// Space Complexity: O(1) - only using constant extra space
fn my_atoi(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let n = chars.len();
    
    // Step 1: Skip leading whitespace
    while i < n && chars[i] == ' ' {
        i += 1;
    }
    
    // Step 2: Handle sign
    let mut sign = 1;
    if i < n && (chars[i] == '+' || chars[i] == '-') {
        if chars[i] == '-' {
            sign = -1;
        }
        i += 1;
    }
    
    // Step 3: Convert digits with overflow detection
    let mut result: i64 = 0;
    const INT_MAX: i64 = i32::MAX as i64; // 2147483647
    const INT_MIN: i64 = i32::MIN as i64; // -2147483648
    
    while i < n && chars[i].is_ascii_digit() {
        let digit = (chars[i] as u8 - b'0') as i64;
        
        // Check for overflow before adding digit
        if result > (INT_MAX - digit) / 10 {
            return if sign == 1 { INT_MAX as i32 } else { INT_MIN as i32 };
        }
        
        result = result * 10 + digit;
        i += 1;
    }
    
    // Apply sign and clamp to 32-bit range
    let final_result = sign as i64 * result;
    final_result.clamp(INT_MIN, INT_MAX) as i32
}

#[derive(Debug)]
struct TestCase {
    input: String,
    expected: i32,
    name: &'static str,
}

fn main() {
    let test_cases = vec![
        TestCase {
            input: "42".to_string(),
            expected: 42,
            name: "Example 1: Simple positive",
        },
        TestCase {
            input: "   -042".to_string(),
            expected: -42,
            name: "Example 2: Leading whitespace and zeros",
        },
        TestCase {
            input: "1337c0d3".to_string(),
            expected: 1337,
            name: "Example 3: Stops at non-digit",
        },
        TestCase {
            input: "0-1".to_string(),
            expected: 0,
            name: "Example 4: Stops at non-digit after zero",
        },
        TestCase {
            input: "words and 987".to_string(),
            expected: 0,
            name: "Example 5: Starts with non-digit",
        },
        TestCase {
            input: "".to_string(),
            expected: 0,
            name: "Empty string",
        },
        TestCase {
            input: "   ".to_string(),
            expected: 0,
            name: "Only whitespace",
        },
        TestCase {
            input: "+".to_string(),
            expected: 0,
            name: "Only sign",
        },
        TestCase {
            input: "-".to_string(),
            expected: 0,
            name: "Only negative sign",
        },
        TestCase {
            input: "2147483647".to_string(),
            expected: 2147483647,
            name: "INT_MAX",
        },
        TestCase {
            input: "2147483648".to_string(),
            expected: 2147483647,
            name: "Overflow to INT_MAX",
        },
        TestCase {
            input: "-2147483648".to_string(),
            expected: -2147483648,
            name: "INT_MIN",
        },
        TestCase {
            input: "-2147483649".to_string(),
            expected: -2147483648,
            name: "Underflow to INT_MIN",
        },
        TestCase {
            input: "9223372036854775808".to_string(),
            expected: 2147483647,
            name: "Very large overflow",
        },
        TestCase {
            input: "  0000000000012345678".to_string(),
            expected: 12345678,
            name: "Leading zeros",
        },
        TestCase {
            input: "+-12".to_string(),
            expected: 0,
            name: "Invalid double sign",
        },
        TestCase {
            input: "   +0 123".to_string(),
            expected: 0,
            name: "Space after digits",
        },
    ];
    
    println!("=== String to Integer (atoi) Test Cases ===\n");
    
    for tc in &test_cases {
        let result = my_atoi(tc.input.clone());
        let status = if result == tc.expected { "✓" } else { "✗" };
        
        println!("{}: input=\"{}\" expected={}, got={} {}", 
            tc.name, tc.input, tc.expected, result, status);
        
        if result != tc.expected {
            println!("  ERROR: Expected {}, but got {}", tc.expected, result);
        }
    }
    
    // Performance test
    println!("\n=== Performance Test ===");
    let performance_cases = vec![
        "1234567890".to_string(),
        "   -987654321".to_string(),
        "2147483647".to_string(),
        "-2147483648".to_string(),
        "99999999999999999999".to_string(),
    ];
    
    for case in performance_cases {
        let start = std::time::Instant::now();
        let result = my_atoi(case.clone());
        let duration = start.elapsed();
        
        println!("Input: \"{}\", Output: {}, Time: {:?}", case, result, duration);
    }
    
    // Edge case analysis
    println!("\n=== Edge Case Analysis ===");
    println!("i32::MAX = {}", i32::MAX);
    println!("i32::MIN = {}", i32::MIN);
    
    let edge_cases = vec![
        ("4193 with words", "4193 with words"),
        ("-91283472332", "-91283472332"),
        ("91283472332", "91283472332"),
        ("20000000000000000000", "20000000000000000000"),
    ];
    
    for (desc, input) in edge_cases {
        let result = my_atoi(input.to_string());
        println!("{}: \"{}\" -> {}", desc, input, result);
    }
} 