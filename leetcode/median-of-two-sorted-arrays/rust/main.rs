/// Median of Two Sorted Arrays - finds median using binary search
/// Time Complexity: O(log(min(m,n))) - binary search on smaller array
/// Space Complexity: O(1) - constant extra space
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (nums1, nums2) = if nums1.len() > nums2.len() {
        (nums2, nums1) // Ensure nums1 is smaller
    } else {
        (nums1, nums2)
    };
    
    let m = nums1.len();
    let n = nums2.len();
    let total = m + n;
    let half = (total + 1) / 2;
    
    let mut left = 0;
    let mut right = m;
    
    loop {
        let partition1 = (left + right) / 2;
        let partition2 = half - partition1;
        
        // Get boundary values
        let max_left1 = if partition1 == 0 { i32::MIN } else { nums1[partition1 - 1] };
        let min_right1 = if partition1 == m { i32::MAX } else { nums1[partition1] };
        
        let max_left2 = if partition2 == 0 { i32::MIN } else { nums2[partition2 - 1] };
        let min_right2 = if partition2 == n { i32::MAX } else { nums2[partition2] };
        
        // Check if we found the correct partition
        if max_left1 <= min_right2 && max_left2 <= min_right1 {
            // Found the correct partition
            if total % 2 == 1 {
                // Odd total length - return middle element
                return max_left1.max(max_left2) as f64;
            } else {
                // Even total length - return average of two middle elements
                let left_max = max_left1.max(max_left2);
                let right_min = min_right1.min(min_right2);
                return (left_max + right_min) as f64 / 2.0;
            }
        } else if max_left1 > min_right2 {
            // Too far on right side of nums1, move left
            right = partition1 - 1;
        } else {
            // Too far on left side of nums1, move right
            left = partition1 + 1;
        }
    }
}

#[derive(Debug)]
struct TestCase {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    expected: f64,
    name: &'static str,
}

fn main() {
    let test_cases = vec![
        TestCase {
            nums1: vec![1, 3],
            nums2: vec![2],
            expected: 2.0,
            name: "Example 1",
        },
        TestCase {
            nums1: vec![1, 2],
            nums2: vec![3, 4],
            expected: 2.5,
            name: "Example 2",
        },
        TestCase {
            nums1: vec![],
            nums2: vec![1],
            expected: 1.0,
            name: "Empty first array",
        },
        TestCase {
            nums1: vec![2],
            nums2: vec![],
            expected: 2.0,
            name: "Empty second array",
        },
        TestCase {
            nums1: vec![1, 3, 5, 7, 9],
            nums2: vec![2, 4, 6, 8, 10],
            expected: 5.5,
            name: "Equal length arrays",
        },
        TestCase {
            nums1: vec![1, 2],
            nums2: vec![1, 2],
            expected: 1.5,
            name: "Duplicate values",
        },
        TestCase {
            nums1: vec![-5, -3, -1],
            nums2: vec![-2, 0, 4],
            expected: -1.5,
            name: "Negative numbers",
        },
    ];
    
    println!("=== Median of Two Sorted Arrays - Binary Search Solution ===\n");
    
    for tc in &test_cases {
        let result = find_median_sorted_arrays(tc.nums1.clone(), tc.nums2.clone());
        let passed = (result - tc.expected).abs() < 1e-9;
        
        println!("{}: nums1={:?}, nums2={:?}", tc.name, tc.nums1, tc.nums2);
        println!("  Expected: {:.5}, Got: {:.5} {}", 
            tc.expected, result, if passed { "✓" } else { "✗" });
        
        if !passed {
            println!("  ERROR: Test case failed!");
        }
        println!();
    }
    
    // Performance test
    println!("=== Performance Test ===");
    let large_nums1: Vec<i32> = (1..=500).step_by(2).collect(); // [1, 3, 5, ...]
    let large_nums2: Vec<i32> = (2..=500).step_by(2).collect(); // [2, 4, 6, ...]
    
    let start = std::time::Instant::now();
    let result = find_median_sorted_arrays(large_nums1.clone(), large_nums2.clone());
    let duration = start.elapsed();
    
    println!("Large input (500 + 500 elements): median = {:.5}", result);
    println!("Execution time: {:?}", duration);
    println!("Time complexity: O(log(min(m,n))) = O(log(250)) ≈ O(8)");
    
    // Edge case performance
    let very_small = vec![1];
    let very_large: Vec<i32> = (2..=1000).collect();
    
    let start = std::time::Instant::now();
    let result = find_median_sorted_arrays(very_small, very_large);
    let duration = start.elapsed();
    
    println!("\nEdge case (1 + 999 elements): median = {:.5}", result);
    println!("Execution time: {:?}", duration);
    println!("Time complexity: O(log(min(1,999))) = O(log(1)) = O(1)");
} 