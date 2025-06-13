use std::collections::HashMap;

/// Two Sum - finds indices of two numbers that add up to target
/// Time Complexity: O(n) - we traverse the array once
/// Space Complexity: O(n) - we store at most n elements in the hash map
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map: HashMap<i32, usize> = HashMap::new();
    
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        
        // Check if complement exists in map
        if let Some(&index) = num_map.get(&complement) {
            return vec![index as i32, i as i32];
        }
        
        // Store current number and its index
        num_map.insert(num, i);
    }
    
    // This should never be reached given the problem constraints
    vec![]
}

#[derive(Debug)]
struct TestCase {
    nums: Vec<i32>,
    target: i32,
    name: &'static str,
}

fn main() {
    let test_cases = vec![
        TestCase {
            nums: vec![2, 7, 11, 15],
            target: 9,
            name: "Example 1",
        },
        TestCase {
            nums: vec![3, 2, 4],
            target: 6,
            name: "Example 2",
        },
        TestCase {
            nums: vec![3, 3],
            target: 6,
            name: "Example 3",
        },
        TestCase {
            nums: vec![-1, -2, -3, -4, -5],
            target: -8,
            name: "Negative numbers",
        },
        TestCase {
            nums: vec![0, 4, 3, 0],
            target: 0,
            name: "Zero target",
        },
    ];
    
    for tc in &test_cases {
        let result = two_sum(tc.nums.clone(), tc.target);
        println!("{}: nums={:?}, target={} -> result={:?}", 
            tc.name, tc.nums, tc.target, result);
        
        // Verify the result
        if result.len() == 2 {
            let idx1 = result[0] as usize;
            let idx2 = result[1] as usize;
            let sum = tc.nums[idx1] + tc.nums[idx2];
            println!("  Verification: nums[{}] + nums[{}] = {} + {} = {} ✓\n", 
                idx1, idx2, tc.nums[idx1], tc.nums[idx2], sum);
        }
    }
    
    // Performance test with larger input
    println!("=== Performance Test ===");
    let large_nums: Vec<i32> = (1..=1000).collect();
    let large_target = 1999; // 999 + 1000
    
    let start = std::time::Instant::now();
    let result = two_sum(large_nums.clone(), large_target);
    let duration = start.elapsed();
    
    println!("Large input (1000 elements): result={:?}", result);
    println!("Execution time: {:?}", duration);
    println!("Average time per element: {:?}", duration / 1000);
}
