/// Helper function to expand around center and return palindrome length
/// Time Complexity: O(n) in worst case
/// Space Complexity: O(1)
fn expand_around_center(chars: &[char], left: i32, right: i32) -> usize {
    let mut l = left;
    let mut r = right;
    
    while l >= 0 && r < chars.len() as i32 && chars[l as usize] == chars[r as usize] {
        l -= 1;
        r += 1;
    }
    
    (r - l - 1) as usize
}

/// Longest Palindromic Substring using Expand Around Centers
/// Time Complexity: O(n²) - we expand around each possible center
/// Space Complexity: O(1) - we only use constant extra space (not counting the result)
fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }
    
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut start = 0;
    let mut max_len = 1;
    
    for i in 0..n {
        // Check for odd-length palindromes (center at i)
        let len1 = expand_around_center(&chars, i as i32, i as i32);
        
        // Check for even-length palindromes (center between i and i+1)
        let len2 = expand_around_center(&chars, i as i32, (i + 1) as i32);
        
        let current_max = len1.max(len2);
        
        if current_max > max_len {
            max_len = current_max;
            start = i - (current_max - 1) / 2;
        }
    }
    
    chars[start..start + max_len].iter().collect()
}

/// Longest Palindromic Substring using Manacher's Algorithm
/// Time Complexity: O(n) - linear time algorithm
/// Space Complexity: O(n) - for preprocessed string and auxiliary arrays
fn longest_palindrome_manacher(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }
    
    // Preprocess the string: "abc" -> "^#a#b#c#$"
    // ^ and $ are sentinels to avoid bounds checking
    let mut processed = String::from("^#");
    for c in s.chars() {
        processed.push(c);
        processed.push('#');
    }
    processed.push('$');
    
    let chars: Vec<char> = processed.chars().collect();
    let n = chars.len();
    let mut p = vec![0; n]; // p[i] = radius of palindrome centered at i
    let mut center = 0;     // center of rightmost palindrome
    let mut right = 0;      // right boundary of rightmost palindrome
    
    for i in 1..n-1 {
        // Mirror of i with respect to center
        let mirror = 2 * center - i;
        
        // If i is within right boundary, we can use previously computed values
        if i < right {
            p[i] = (right - i).min(p[mirror]);
        }
        
        // Try to expand palindrome centered at i
        while chars[i + p[i] + 1] == chars[i - p[i] - 1] {
            p[i] += 1;
        }
        
        // If palindrome centered at i extends past right, update center and right
        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }
    }
    
    // Find the longest palindrome
    let mut max_len = 0;
    let mut center_index = 0;
    for i in 1..n-1 {
        if p[i] > max_len {
            max_len = p[i];
            center_index = i;
        }
    }
    
    // Extract the palindrome from original string
    let start = (center_index - max_len) / 2;
    s.chars().skip(start).take(max_len).collect()
}

#[derive(Debug)]
struct TestCase {
    input: String,
    expected: Vec<String>, // Multiple valid answers possible
    name: &'static str,
}

fn main() {
    let test_cases = vec![
        TestCase {
            input: "babad".to_string(),
            expected: vec!["bab".to_string(), "aba".to_string()],
            name: "Example 1",
        },
        TestCase {
            input: "cbbd".to_string(),
            expected: vec!["bb".to_string()],
            name: "Example 2",
        },
        TestCase {
            input: "a".to_string(),
            expected: vec!["a".to_string()],
            name: "Single character",
        },
        TestCase {
            input: "ac".to_string(),
            expected: vec!["a".to_string(), "c".to_string()],
            name: "Two different characters",
        },
        TestCase {
            input: "racecar".to_string(),
            expected: vec!["racecar".to_string()],
            name: "Full palindrome",
        },
        TestCase {
            input: "noon".to_string(),
            expected: vec!["noon".to_string()],
            name: "Even-length palindrome",
        },
        TestCase {
            input: "abcdcba".to_string(),
            expected: vec!["abcdcba".to_string()],
            name: "Odd-length full palindrome",
        },
        TestCase {
            input: "bananas".to_string(),
            expected: vec!["anana".to_string()],
            name: "Embedded palindrome",
        },
    ];
    
    println!("=== Testing Both Algorithms ===\n");
    
    for tc in &test_cases {
        let result1 = longest_palindrome(tc.input.clone());
        let result2 = longest_palindrome_manacher(tc.input.clone());
        
        let is_valid1 = tc.expected.contains(&result1);
        let is_valid2 = tc.expected.contains(&result2);
        
        println!("{}: input=\"{}\"", tc.name, tc.input);
        println!("  Expand Around Centers: \"{}\" {}", result1, if is_valid1 { "✓" } else { "✗" });
        println!("  Manacher's Algorithm:  \"{}\" {}\n", result2, if is_valid2 { "✓" } else { "✗" });
    }
    
    // Performance comparison
    println!("=== Performance Comparison ===");
    
    // Test different sizes
    let sizes = vec![100, 1000, 5000, 10000];
    
    for size in sizes {
        println!("\nTesting with size: {}", size);
        
        // Create test string with embedded palindrome
        let test_input = "a".repeat(size / 2) + "racecar" + &"b".repeat(size / 2);
        
        // Test Expand Around Centers (O(n²))
        let start = std::time::Instant::now();
        let result1 = longest_palindrome(test_input.clone());
        let duration1 = start.elapsed();
        
        // Test Manacher's Algorithm (O(n))
        let start = std::time::Instant::now();
        let result2 = longest_palindrome_manacher(test_input.clone());
        let duration2 = start.elapsed();
        
        println!("  Expand Around Centers (O(n²)): {:>8.3}ms -> \"{}\"", 
            duration1.as_secs_f64() * 1000.0, 
            if result1.len() > 20 { format!("{}...", &result1[..20]) } else { result1 });
        
        println!("  Manacher's Algorithm  (O(n)) : {:>8.3}ms -> \"{}\"", 
            duration2.as_secs_f64() * 1000.0, 
            if result2.len() > 20 { format!("{}...", &result2[..20]) } else { result2 });
        
        let speedup = duration1.as_secs_f64() / duration2.as_secs_f64();
        if speedup > 1.0 {
            println!("  Manacher is {:.2}x faster", speedup);
        } else {
            println!("  Expand Around Centers is {:.2}x faster", 1.0 / speedup);
        }
    }
} 