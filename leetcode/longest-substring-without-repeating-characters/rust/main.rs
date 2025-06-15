use std::collections::HashMap;

/// Optimal solution using sliding window with HashMap
/// Time complexity: O(n)
/// Space complexity: O(min(m,n)) where m is the size of the charset
fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut char_index: HashMap<char, usize> = HashMap::new();
    let mut max_length = 0;
    let mut start = 0;
    
    for (end, &ch) in chars.iter().enumerate() {
        // If character is already seen and is within current window
        if let Some(&prev_index) = char_index.get(&ch) {
            if prev_index >= start {
                start = prev_index + 1;
            }
        }
        
        // Update character's latest index
        char_index.insert(ch, end);
        
        // Update maximum length
        max_length = max_length.max(end - start + 1);
    }
    
    max_length as i32
}

/// OPTIMIZED: Direct string iteration like Go - no Vec allocation!
/// This should be much faster and comparable to Go performance
fn length_of_longest_substring_optimized(s: &str) -> i32 {
    let mut char_index: HashMap<char, usize> = HashMap::new();
    let mut max_length = 0;
    let mut start = 0;
    
    for (end, ch) in s.chars().enumerate() {
        // If character is already seen and is within current window
        if let Some(&prev_index) = char_index.get(&ch) {
            if prev_index >= start {
                start = prev_index + 1;
            }
        }
        
        // Update character's latest index
        char_index.insert(ch, end);
        
        // Update maximum length
        max_length = max_length.max(end - start + 1);
    }
    
    max_length as i32
}

/// Alternative solution using sliding window with character frequency
/// More space-efficient for ASCII characters
fn length_of_longest_substring_array(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut char_last_index: [i32; 128] = [-1; 128]; // ASCII characters
    let mut max_length = 0;
    let mut start = 0;
    
    for (end, &ch) in chars.iter().enumerate() {
        let char_code = ch as usize;
        
        // Check if character is within ASCII range
        if char_code < 128 {
            let last_index = char_last_index[char_code];
            
            // If character is already seen and is within current window
            if last_index >= start as i32 {
                start = (last_index + 1) as usize;
            }
            
            // Update character's latest index
            char_last_index[char_code] = end as i32;
        }
        
        // Update maximum length
        max_length = max_length.max(end - start + 1);
    }
    
    max_length as i32
}

/// OPTIMIZED ASCII version: Direct string iteration - no Vec allocation!
fn length_of_longest_substring_array_optimized(s: &str) -> i32 {
    let mut char_last_index: [i32; 128] = [-1; 128]; // ASCII characters
    let mut max_length = 0;
    let mut start = 0;
    
    for (end, ch) in s.chars().enumerate() {
        let char_code = ch as usize;
        
        // Check if character is within ASCII range
        if char_code < 128 {
            let last_index = char_last_index[char_code];
            
            // If character is already seen and is within current window
            if last_index >= start as i32 {
                start = (last_index + 1) as usize;
            }
            
            // Update character's latest index
            char_last_index[char_code] = end as i32;
        }
        
        // Update maximum length  
        max_length = max_length.max(end - start + 1);
    }
    
    max_length as i32
}

fn main() {
    // Test cases  
    let test_cases = vec![
        ("abcabcbb".to_string(), 3),
        ("bbbbb".to_string(), 1),
        ("pwwkew".to_string(), 3),
        ("".to_string(), 0),
        ("au".to_string(), 2),
        ("dvdf".to_string(), 3),
        ("abba".to_string(), 2),
        ("tmmzuxt".to_string(), 5),
    ];
    
    println!("Testing Longest Substring Without Repeating Characters");
    println!("{}", "=".repeat(60));
    
    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let result1 = length_of_longest_substring(input.clone());
        let result2 = length_of_longest_substring_array(input.clone());
        let result3 = length_of_longest_substring_optimized(input);
        let result4 = length_of_longest_substring_array_optimized(input);
        
        println!("Test {}: '{}'", i + 1, input);
        println!("  Expected: {}", expected);
        println!("  HashMap:        {} {}", result1, if result1 == *expected { "✓" } else { "✗" });
        println!("  Array:          {} {}", result2, if result2 == *expected { "✓" } else { "✗" });
        println!("  HashMap (Opt):  {} {}", result3, if result3 == *expected { "✓" } else { "✗" });
        println!("  Array (Opt):    {} {}", result4, if result4 == *expected { "✓" } else { "✗" });
        println!();
    }
    
    // Performance test
    println!("Performance Test:");
    println!("{}", "-".repeat(30));
    
    use std::time::Instant;
    
    let large_string = "abcdefghijklmnopqrstuvwxyz".repeat(1000);
    
    let start_time = Instant::now();
    let result = length_of_longest_substring(large_string.clone());
    let duration = start_time.elapsed();
    
    println!("Large string (26000 chars): {} ms", duration.as_millis());
    println!("Result: {}", result);
    
    // Micro-benchmark: measure only the algorithm execution
    println!("\n=== Micro-benchmark ===");
    
    // Pre-create test data to avoid measuring allocation in the loop
    let test_strings = vec![
        "abcabcbb".to_string(),
        "bbbbb".to_string(),
        "pwwkew".to_string(),
        "abcdefghijklmnopqrstuvwxyz".to_string(),
        "aab".to_string(),
        "dvdf".to_string(),
        "tmmzuxt".to_string(),
        "abba".to_string(),
        "au".to_string(),
        "".to_string(),
    ];
    
    let iterations = 100_000;
    
    // Test HashMap implementation (with Vec allocation)
    let start = Instant::now();
    for i in 0..iterations {
        let test_string = &test_strings[i % test_strings.len()];
        let _result = length_of_longest_substring(test_string.clone());
    }
    let duration = start.elapsed();
    let avg_time = duration / iterations as u32;
    println!("HashMap (Vec) - Average time per operation: {:?}", avg_time);
    println!("HashMap (Vec) - Operations per second: {:.0}", 1.0 / avg_time.as_secs_f64());
    
    // Test OPTIMIZED HashMap implementation (direct iteration)
    let start = Instant::now();
    for i in 0..iterations {
        let test_string = &test_strings[i % test_strings.len()];
        let _result = length_of_longest_substring_optimized(test_string);
    }
    let duration = start.elapsed();
    let avg_time = duration / iterations as u32;
    println!("HashMap (Opt) - Average time per operation: {:?}", avg_time);
    println!("HashMap (Opt) - Operations per second: {:.0}", 1.0 / avg_time.as_secs_f64());
    
    // Test Array implementation (with Vec allocation)
    let start = Instant::now();
    for i in 0..iterations {
        let test_string = &test_strings[i % test_strings.len()];
        let _result = length_of_longest_substring_array(test_string.clone());
    }
    let duration = start.elapsed();
    let avg_time = duration / iterations as u32;
    println!("Array (Vec) - Average time per operation: {:?}", avg_time);
    println!("Array (Vec) - Operations per second: {:.0}", 1.0 / avg_time.as_secs_f64());
    
    // Test OPTIMIZED Array implementation (direct iteration)
    let start = Instant::now();
    for i in 0..iterations {
        let test_string = &test_strings[i % test_strings.len()];
        let _result = length_of_longest_substring_array_optimized(test_string);
    }
    let duration = start.elapsed();
    let avg_time = duration / iterations as u32;
    println!("Array (Opt) - Average time per operation: {:?}", avg_time);
    println!("Array (Opt) - Operations per second: {:.0}", 1.0 / avg_time.as_secs_f64());
    
    // Memory usage test
    println!("\n=== Memory Usage Test ===");
    let huge_string = "abcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()".repeat(1000);
    println!("Huge string length: {} characters", huge_string.len());
    
    let start = Instant::now();
    let result_hashmap = length_of_longest_substring(huge_string.clone());
    let time_hashmap = start.elapsed();
    
    let start = Instant::now();
    let result_hashmap_opt = length_of_longest_substring_optimized(&huge_string);
    let time_hashmap_opt = start.elapsed();
    
    let start = Instant::now();
    let result_array = length_of_longest_substring_array(huge_string.clone());
    let time_array = start.elapsed();
    
    let start = Instant::now();
    let result_array_opt = length_of_longest_substring_array_optimized(&huge_string);
    let time_array_opt = start.elapsed();
    
    println!("HashMap (Vec) result: {} (time: {:?})", result_hashmap, time_hashmap);
    println!("HashMap (Opt) result: {} (time: {:?})", result_hashmap_opt, time_hashmap_opt);
    println!("Array (Vec) result: {} (time: {:?})", result_array, time_array);
    println!("Array (Opt) result: {} (time: {:?})", result_array_opt, time_array_opt);
    
    // Test with different character sets
    println!("\n=== Character Set Performance ===");
    
    let ascii_string = "The quick brown fox jumps over the lazy dog 1234567890".repeat(100);
    let unicode_string = "Привет мир! Hello world! 你好世界! こんにちは世界!".repeat(100);
    let mixed_string = "abc123!@#АБВ你好こんにちは".repeat(200);
    
    let test_cases = vec![
        ("ASCII", ascii_string),
        ("Unicode", unicode_string),
        ("Mixed", mixed_string),
    ];
    
    for (name, test_string) in test_cases {
        let start = Instant::now();
        let result = length_of_longest_substring(test_string.clone());
        let duration = start.elapsed();
        
        let start_opt = Instant::now();
        let result_opt = length_of_longest_substring_optimized(&test_string);
        let duration_opt = start_opt.elapsed();
        
        println!("{} ({} chars):", name, test_string.chars().count());
        println!("  Vec version:  result={}, time={:?}", result, duration);
        println!("  Opt version:  result={}, time={:?}", result_opt, duration_opt);
    }
    
    // Worst case scenario test
    println!("\n=== Worst Case Scenario ===");
    let worst_case = "a".repeat(10000); // All same characters
    let best_case: String = (0..10000).map(|i| char::from((i % 26 + 97) as u8)).collect(); // All different
    
    let start = Instant::now();
    let worst_result = length_of_longest_substring(worst_case.clone());
    let worst_time = start.elapsed();
    
    let start = Instant::now();
    let worst_result_opt = length_of_longest_substring_optimized(&worst_case);
    let worst_time_opt = start.elapsed();
    
    let start = Instant::now();
    let best_result = length_of_longest_substring(best_case.clone());
    let best_time = start.elapsed();
    
    let start = Instant::now();
    let best_result_opt = length_of_longest_substring_optimized(&best_case);
    let best_time_opt = start.elapsed();
    
    println!("Worst case (all same):");
    println!("  Vec version: result={}, time={:?}", worst_result, worst_time);
    println!("  Opt version: result={}, time={:?}", worst_result_opt, worst_time_opt);
    println!("Best case (all different):");
    println!("  Vec version: result={}, time={:?}", best_result, best_time);
    println!("  Opt version: result={}, time={:?}", best_result_opt, best_time_opt);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
        assert_eq!(length_of_longest_substring("au".to_string()), 2);
        assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
        assert_eq!(length_of_longest_substring("abba".to_string()), 2);
        assert_eq!(length_of_longest_substring("tmmzuxt".to_string()), 5);
    }
    
    #[test]
    fn test_length_of_longest_substring_array() {
        assert_eq!(length_of_longest_substring_array("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring_array("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring_array("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring_array("".to_string()), 0);
        assert_eq!(length_of_longest_substring_array("au".to_string()), 2);
        assert_eq!(length_of_longest_substring_array("dvdf".to_string()), 3);
    }
    
    #[test]
    fn test_edge_cases() {
        assert_eq!(length_of_longest_substring("a".to_string()), 1);
        assert_eq!(length_of_longest_substring("ab".to_string()), 2);
        assert_eq!(length_of_longest_substring("aa".to_string()), 1);
        assert_eq!(length_of_longest_substring("aabaab!bb".to_string()), 3);
    }
} 