/// Reverse Integer - reverses digits of a 32-bit signed integer
/// Time Complexity: O(log n) - where n is the input number (number of digits)
/// Space Complexity: O(1) - only using constant extra space
fn reverse(x: i32) -> i32 {
    let mut num = x;
    let mut result: i32 = 0;
    
    // Constants for 32-bit signed integer bounds
    const INT_MAX: i32 = i32::MAX; // 2147483647
    const INT_MIN: i32 = i32::MIN; // -2147483648
    
    while num != 0 {
        let digit = num % 10;
        num /= 10;
        
        // Check for overflow before multiplying and adding
        // For positive overflow: result > (INT_MAX - digit) / 10
        // For negative overflow: result < (INT_MIN - digit) / 10
        if result > INT_MAX / 10 || (result == INT_MAX / 10 && digit > 7) {
            return 0;
        }
        if result < INT_MIN / 10 || (result == INT_MIN / 10 && digit < -8) {
            return 0;
        }
        
        result = result * 10 + digit;
    }
    
    result
}

#[derive(Debug)]
struct TestCase {
    input: i32,
    expected: i32,
    name: &'static str,
}

fn main() {
    let test_cases = vec![
        TestCase {
            input: 123,
            expected: 321,
            name: "Example 1: Positive number",
        },
        TestCase {
            input: -123,
            expected: -321,
            name: "Example 2: Negative number",
        },
        TestCase {
            input: 120,
            expected: 21,
            name: "Example 3: Trailing zeros",
        },
        TestCase {
            input: 0,
            expected: 0,
            name: "Zero",
        },
        TestCase {
            input: 1534236469,
            expected: 0,
            name: "Overflow case",
        },
        TestCase {
            input: -2147483648,
            expected: 0,
            name: "INT_MIN overflow",
        },
        TestCase {
            input: 1463847412,
            expected: 2147483641,
            name: "Near overflow boundary",
        },
        TestCase {
            input: -1463847412,
            expected: -2147483641,
            name: "Near negative overflow boundary",
        },
    ];
    
    println!("=== Reverse Integer Test Cases ===\n");
    
    for tc in &test_cases {
        let result = reverse(tc.input);
        let status = if result == tc.expected { "✓" } else { "✗" };
        
        println!("{}: input={}, expected={}, got={} {}", 
            tc.name, tc.input, tc.expected, result, status);
        
        if result != tc.expected {
            println!("  ERROR: Expected {}, but got {}", tc.expected, result);
        }
    }
    
    // Performance test
    println!("\n=== Performance Test ===");
    let performance_cases = vec![1000000000, -1000000000, 123456789, -987654321];
    
    for case in performance_cases {
        let start = std::time::Instant::now();
        let result = reverse(case);
        let duration = start.elapsed();
        
        println!("Input: {}, Output: {}, Time: {:?}", case, result, duration);
    }
    
    // Edge case analysis
    println!("\n=== Edge Case Analysis ===");
    println!("i32::MAX = {}", i32::MAX);
    println!("i32::MIN = {}", i32::MIN);
    println!("Reverse of i32::MAX should overflow: {}", reverse(i32::MAX));
    println!("Reverse of i32::MIN should overflow: {}", reverse(i32::MIN));
} 