/// ZigZag Conversion - transforms string into zigzag pattern and reads line by line
/// Time Complexity: O(n) - we visit each character exactly once
/// Space Complexity: O(n) - we store characters in rows
fn convert(s: String, num_rows: i32) -> String {
    let num_rows = num_rows as usize;
    
    // Edge case: if numRows is 1 or string is shorter than numRows
    if num_rows == 1 || s.len() <= num_rows {
        return s;
    }

    // Create a vector of strings for each row
    let mut rows: Vec<String> = vec![String::new(); num_rows];
    let mut current_row = 0;
    let mut going_down = false;

    // Traverse each character in the string
    for ch in s.chars() {
        // Add character to current row
        rows[current_row].push(ch);

        // Change direction when reaching top or bottom
        if current_row == 0 || current_row == num_rows - 1 {
            going_down = !going_down;
        }

        // Move to next row based on direction
        if going_down {
            current_row += 1;
        } else {
            current_row -= 1;
        }
    }

    // Concatenate all rows
    rows.join("")
}

/// Alternative mathematical approach - more space efficient
/// Time Complexity: O(n) - we visit each character position once
/// Space Complexity: O(1) - only using constant extra space (excluding result)
fn convert_math(s: String, num_rows: i32) -> String {
    let num_rows = num_rows as usize;
    
    if num_rows == 1 || s.len() <= num_rows {
        return s;
    }

    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let cycle_len = 2 * num_rows - 2; // Length of one complete zigzag cycle

    for i in 0..num_rows {
        let mut j = 0;
        while j + i < n {
            // Add character from current position
            result.push(chars[j + i]);

            // Add middle character (if not first or last row)
            if i != 0 && i != num_rows - 1 && j + cycle_len - i < n {
                result.push(chars[j + cycle_len - i]);
            }

            j += cycle_len;
        }
    }

    result
}

#[derive(Debug, Clone)]
struct TestCase {
    s: String,
    num_rows: i32,
    name: &'static str,
}

fn main() {
    // Test cases
    let test_cases = vec![
        TestCase {
            s: "PAYPALISHIRING".to_string(),
            num_rows: 3,
            name: "Example 1",
        },
        TestCase {
            s: "PAYPALISHIRING".to_string(),
            num_rows: 4,
            name: "Example 2",
        },
        TestCase {
            s: "A".to_string(),
            num_rows: 1,
            name: "Example 3",
        },
        TestCase {
            s: "AB".to_string(),
            num_rows: 1,
            name: "Single row",
        },
        TestCase {
            s: "ABCDEFGHIJKLMNOP".to_string(),
            num_rows: 4,
            name: "Longer string",
        },
        TestCase {
            s: "".to_string(),
            num_rows: 3,
            name: "Empty string",
        },
        TestCase {
            s: "ABCD".to_string(),
            num_rows: 2,
            name: "Simple pattern",
        },
        TestCase {
            s: "ABCDEFGHIJ".to_string(),
            num_rows: 5,
            name: "Multiple cycles",
        },
    ];

    println!("=== ZigZag Conversion Solutions ===\n");

    for tc in &test_cases {
        let result1 = convert(tc.s.clone(), tc.num_rows);
        let result2 = convert_math(tc.s.clone(), tc.num_rows);

        println!("{}:", tc.name);
        println!("  Input: s = \"{}\", numRows = {}", tc.s, tc.num_rows);
        println!("  Output (Row-by-row): \"{}\"", result1);
        println!("  Output (Mathematical): \"{}\"", result2);

        // Verify both solutions match
        if result1 == result2 {
            println!("  ✓ Both solutions match");
        } else {
            println!("  ✗ Solutions differ!");
        }

        // Show pattern visualization for small strings
        if tc.s.len() <= 20 && tc.num_rows > 1 {
            visualize_pattern(&tc.s, tc.num_rows as usize);
        }
        println!();
    }

    // Performance comparison
    println!("=== Performance Test ===");
    let large_string = "ABCDEFGHIJ".repeat(1000); // 10,000 characters
    let num_rows = 7;

    // Test row-by-row approach
    let start = std::time::Instant::now();
    let result1 = convert(large_string.clone(), num_rows);
    let duration1 = start.elapsed();

    // Test mathematical approach
    let start = std::time::Instant::now();
    let result2 = convert_math(large_string, num_rows);
    let duration2 = start.elapsed();

    println!("Large string (10,000 chars), numRows = {}:", num_rows);
    println!("  Row-by-row approach: {:?}", duration1);
    println!("  Mathematical approach: {:?}", duration2);
    println!("  Results match: {}", result1 == result2);
    
    if duration2.as_nanos() > 0 {
        let speedup = duration1.as_nanos() as f64 / duration2.as_nanos() as f64;
        println!("  Mathematical is {:.2}x faster", speedup);
    }

    // Memory usage demonstration
    println!("\n=== Memory Usage Analysis ===");
    demonstrate_memory_usage();
}

/// Helper function to visualize the zigzag pattern
fn visualize_pattern(s: &str, num_rows: usize) {
    if num_rows == 1 {
        return;
    }

    println!("  Pattern visualization:");
    let chars: Vec<char> = s.chars().collect();
    let mut rows: Vec<Vec<char>> = vec![vec![' '; s.len()]; num_rows];
    
    let mut current_row = 0;
    let mut going_down = false;

    for (col, &ch) in chars.iter().enumerate() {
        rows[current_row][col] = ch;

        if current_row == 0 || current_row == num_rows - 1 {
            going_down = !going_down;
        }

        if going_down {
            current_row += 1;
        } else {
            current_row -= 1;
        }
    }

    // Print the pattern
    for row in &rows {
        print!("    ");
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

/// Demonstrate memory usage patterns for both approaches
fn demonstrate_memory_usage() {
    let test_string = "ABCDEFGHIJ".repeat(100); // 1,000 characters
    let num_rows = 5;

    println!("Testing with string length: {}", test_string.len());
    println!("Number of rows: {}", num_rows);
    
    // Row-by-row approach memory analysis
    println!("\nRow-by-row approach:");
    println!("  - Creates {} separate strings (one per row)", num_rows);
    println!("  - Each row stores approximately {} characters", test_string.len() / num_rows);
    println!("  - Total extra memory: ~{} bytes for row storage", test_string.len());
    
    // Mathematical approach memory analysis
    println!("\nMathematical approach:");
    println!("  - Creates character vector: {} bytes", test_string.len() * 4); // 4 bytes per char
    println!("  - Builds result string incrementally");
    println!("  - More predictable memory usage pattern");

    // Test both approaches
    let start_mem = get_memory_usage();
    let _result1 = convert(test_string.clone(), num_rows as i32);
    let mem_after_row = get_memory_usage();
    
    let _result2 = convert_math(test_string, num_rows as i32);
    let mem_after_math = get_memory_usage();

    println!("\nMemory change (approximate):");
    println!("  After row-by-row: +{} KB", mem_after_row.saturating_sub(start_mem) / 1024);
    println!("  After mathematical: +{} KB", mem_after_math.saturating_sub(mem_after_row) / 1024);
}

/// Simplified memory usage estimation (not perfectly accurate)
fn get_memory_usage() -> usize {
    // This is a simplified approximation
    // In a real application, you'd use proper memory profiling tools
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as usize % 1024
} 