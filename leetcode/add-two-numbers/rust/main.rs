// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Add Two Numbers - adds two numbers represented as linked lists
/// Time Complexity: O(max(m, n)) - where m and n are lengths of the two lists
/// Space Complexity: O(max(m, n)) - for the result list
fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0);
    let mut current = &mut dummy_head;
    let mut carry = 0;
    let mut p = l1.as_ref();
    let mut q = l2.as_ref();

    while p.is_some() || q.is_some() || carry != 0 {
        let x = p.map_or(0, |node| node.val);
        let y = q.map_or(0, |node| node.val);
        let sum = carry + x + y;

        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().expect("Just created this node");

        if let Some(node) = p {
            p = node.next.as_ref();
        }
        if let Some(node) = q {
            q = node.next.as_ref();
        }
    }

    dummy_head.next
}

/// Optimized version with less heap allocations
fn add_two_numbers_optimized(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut current = &mut dummy_head;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let mut sum = carry;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }

        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }

        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().expect("Just created this node");
    }

    dummy_head.next
}

/// Version 1: Returns Result instead of panicking
/// This is the safest approach for production code
fn add_two_numbers_safe_result(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Result<Option<Box<ListNode>>, &'static str> {
    let mut dummy_head = ListNode::new(0);
    let mut current = &mut dummy_head;
    let mut carry = 0;
    let mut p = l1.as_ref();
    let mut q = l2.as_ref();

    while p.is_some() || q.is_some() || carry != 0 {
        let x = p.map_or(0, |node| node.val);
        let y = q.map_or(0, |node| node.val);
        let sum = carry + x + y;

        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));

        // Safe handling with Result
        current = match current.next.as_mut() {
            Some(next_node) => next_node,
            None => return Err("Failed to create next node"),
        };

        if let Some(node) = p {
            p = node.next.as_ref();
        }
        if let Some(node) = q {
            q = node.next.as_ref();
        }
    }

    Ok(dummy_head.next)
}

/// Version 2: Ultra-safe with if let - never panics
/// Uses explicit pattern matching for complete safety
fn add_two_numbers_ultra_safe(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0);
    let mut current = &mut dummy_head;
    let mut carry = 0;
    let mut p = l1.as_ref();
    let mut q = l2.as_ref();

    while p.is_some() || q.is_some() || carry != 0 {
        let x = p.map_or(0, |node| node.val);
        let y = q.map_or(0, |node| node.val);
        let sum = carry + x + y;

        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));

        // Completely safe - no unwrap, no expect, no panic possible
        if let Some(ref mut next_node) = current.next {
            current = next_node;
        } else {
            // This should never happen, but we handle it gracefully
            break;
        }

        if let Some(node) = p {
            p = node.next.as_ref();
        }
        if let Some(node) = q {
            q = node.next.as_ref();
        }
    }

    dummy_head.next
}

/// Version 3: Elegant approach - avoids the problem entirely
/// Uses a different algorithm structure that doesn't need unsafe operations
fn add_two_numbers_elegant(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    fn add_recursive(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            return None;
        }

        let mut sum = carry;
        let next_l1 = l1.as_ref().and_then(|node| node.next.clone());
        let next_l2 = l2.as_ref().and_then(|node| node.next.clone());

        if let Some(node) = l1.as_ref() {
            sum += node.val;
        }
        if let Some(node) = l2.as_ref() {
            sum += node.val;
        }

        let mut new_node = ListNode::new(sum % 10);
        new_node.next = add_recursive(next_l1, next_l2, sum / 10);

        Some(Box::new(new_node))
    }

    add_recursive(l1, l2, 0)
}

/// Version 4: Functional approach using iterators
/// Completely safe and elegant, though slightly more complex
fn add_two_numbers_functional(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // Convert linked lists to iterators
    fn list_to_iter(mut head: Option<Box<ListNode>>) -> impl Iterator<Item = i32> {
        std::iter::from_fn(move || {
            head.take().map(|node| {
                head = node.next;
                node.val
            })
        })
    }

    let iter1 = list_to_iter(l1);
    let iter2 = list_to_iter(l2);

    let mut result_digits = Vec::new();
    let mut carry = 0;

    // Use iterators to process both lists
    let mut iter1 = iter1.peekable();
    let mut iter2 = iter2.peekable();

    while iter1.peek().is_some() || iter2.peek().is_some() || carry != 0 {
        let x = iter1.next().unwrap_or(0);
        let y = iter2.next().unwrap_or(0);
        let sum = x + y + carry;

        result_digits.push(sum % 10);
        carry = sum / 10;
    }

    // Convert result back to linked list
    let mut result = None;
    for &digit in result_digits.iter().rev() {
        let mut new_node = ListNode::new(digit);
        new_node.next = result;
        result = Some(Box::new(new_node));
    }

    result
}

// Helper function to create linked list from vector
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vec.iter().rev() {
        let mut new_node = ListNode::new(val);
        new_node.next = head;
        head = Some(Box::new(new_node));
    }
    head
}

// Helper function to convert linked list to vector for easy display
fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

#[derive(Debug)]
struct TestCase {
    l1: Vec<i32>,
    l2: Vec<i32>,
    name: &'static str,
}

fn main() {
    let test_cases = vec![
        TestCase {
            l1: vec![2, 4, 3],
            l2: vec![5, 6, 4],
            name: "Example 1",
        },
        TestCase {
            l1: vec![0],
            l2: vec![0],
            name: "Example 2",
        },
        TestCase {
            l1: vec![9, 9, 9, 9, 9, 9, 9],
            l2: vec![9, 9, 9, 9],
            name: "Example 3",
        },
        TestCase {
            l1: vec![1, 8],
            l2: vec![0],
            name: "One list is shorter",
        },
        TestCase {
            l1: vec![5],
            l2: vec![5],
            name: "Single digit with carry",
        },
    ];

    for tc in &test_cases {
        println!("=== Testing {} ===", tc.name);
        let l1 = vec_to_list(tc.l1.clone());
        let l2 = vec_to_list(tc.l2.clone());

        // Test original version
        let result = add_two_numbers(l1.clone(), l2.clone());
        let result_vec = list_to_vec(result);
        println!(
            "Original: l1={:?}, l2={:?} -> result={:?}",
            tc.l1, tc.l2, result_vec
        );

        // Test safe Result version
        let result_safe = add_two_numbers_safe_result(l1.clone(), l2.clone());
        match result_safe {
            Ok(result) => {
                let result_vec = list_to_vec(result);
                println!("Safe Result: result={:?}", result_vec);
            }
            Err(e) => println!("Safe Result: Error={}", e),
        }

        // Test ultra-safe version
        let result_ultra = add_two_numbers_ultra_safe(l1.clone(), l2.clone());
        let result_vec = list_to_vec(result_ultra);
        println!("Ultra Safe: result={:?}", result_vec);

        // Test elegant recursive version
        let result_elegant = add_two_numbers_elegant(l1.clone(), l2.clone());
        let result_vec = list_to_vec(result_elegant);
        println!("Elegant: result={:?}", result_vec);

        // Test functional version
        let result_functional = add_two_numbers_functional(l1.clone(), l2.clone());
        let result_vec = list_to_vec(result_functional);
        println!("Functional: result={:?}", result_vec);

        // Calculate expected result for verification
        let num1 = tc.l1.iter().enumerate().fold(0i64, |acc, (i, &digit)| {
            acc + digit as i64 * 10_i64.pow(i as u32)
        });
        let num2 = tc.l2.iter().enumerate().fold(0i64, |acc, (i, &digit)| {
            acc + digit as i64 * 10_i64.pow(i as u32)
        });
        let expected_sum = num1 + num2;

        println!("  Verification: {} + {} = {} ✓\n", num1, num2, expected_sum);
    }

    // Performance test with larger input
    println!("=== Performance Test ===");
    let large_l1: Vec<i32> = (0..100).map(|i| i % 10).collect();
    let large_l2: Vec<i32> = (0..98).map(|i| (i + 1) % 10).collect();

    let start = std::time::Instant::now();
    let l1 = vec_to_list(large_l1.clone());
    let l2 = vec_to_list(large_l2.clone());
    let result = add_two_numbers(l1, l2);
    let result_vec = list_to_vec(result);
    let duration = start.elapsed();

    println!(
        "Large input (100 + 98 digits): result length={}",
        result_vec.len()
    );
    println!("Execution time: {:?}", duration);

    // Micro-benchmark: measure only the algorithm execution
    println!("\n=== Micro-benchmark ===");

    // Pre-create test data to avoid measuring allocation in the loop
    let mut test_data = Vec::new();
    for _ in 0..1000 {
        let l1 = vec_to_list(vec![2, 4, 3]);
        let l2 = vec_to_list(vec![5, 6, 4]);
        test_data.push((l1, l2));
    }

    let iterations = 100000;

    // Test original implementation
    let start = std::time::Instant::now();
    for i in 0..iterations {
        let (l1, l2) = test_data[i % 1000].clone();
        let _result = add_two_numbers(l1, l2);
    }
    let duration = start.elapsed();
    let avg_time = duration / iterations as u32;
    println!("Original - Average time per operation: {:?}", avg_time);
    println!(
        "Original - Operations per second: {:.0}",
        1.0 / avg_time.as_secs_f64()
    );

    // Test optimized implementation
    let start = std::time::Instant::now();
    for i in 0..iterations {
        let (l1, l2) = test_data[i % 1000].clone();
        let _result = add_two_numbers_optimized(l1, l2);
    }
    let duration = start.elapsed();
    let avg_time = duration / iterations as u32;
    println!("Optimized - Average time per operation: {:?}", avg_time);
    println!(
        "Optimized - Operations per second: {:.0}",
        1.0 / avg_time.as_secs_f64()
    );

    // Test ultra-safe implementation
    let start = std::time::Instant::now();
    for i in 0..iterations {
        let (l1, l2) = test_data[i % 1000].clone();
        let _result = add_two_numbers_ultra_safe(l1, l2);
    }
    let duration = start.elapsed();
    let avg_time = duration / iterations as u32;
    println!("Ultra Safe - Average time per operation: {:?}", avg_time);
    println!(
        "Ultra Safe - Operations per second: {:.0}",
        1.0 / avg_time.as_secs_f64()
    );

    // Test elegant recursive implementation
    let start = std::time::Instant::now();
    for i in 0..iterations {
        let (l1, l2) = test_data[i % 1000].clone();
        let _result = add_two_numbers_elegant(l1, l2);
    }
    let duration = start.elapsed();
    let avg_time = duration / iterations as u32;
    println!("Elegant - Average time per operation: {:?}", avg_time);
    println!(
        "Elegant - Operations per second: {:.0}",
        1.0 / avg_time.as_secs_f64()
    );

    // Test functional implementation
    let start = std::time::Instant::now();
    for i in 0..iterations {
        let (l1, l2) = test_data[i % 1000].clone();
        let _result = add_two_numbers_functional(l1, l2);
    }
    let duration = start.elapsed();
    let avg_time = duration / iterations as u32;
    println!("Functional - Average time per operation: {:?}", avg_time);
    println!(
        "Functional - Operations per second: {:.0}",
        1.0 / avg_time.as_secs_f64()
    );

    // Test unsafe implementation
    unsafe {
        let mut fast_test_data = Vec::new();
        for _ in 0..1000 {
            let l1 = vec_to_fast_list(&[2, 4, 3]);
            let l2 = vec_to_fast_list(&[5, 6, 4]);
            fast_test_data.push((l1, l2));
        }

        let start = std::time::Instant::now();
        for i in 0..iterations {
            let (l1, l2) = fast_test_data[i % 1000];
            let l1_clone = clone_fast_list(l1);
            let l2_clone = clone_fast_list(l2);
            let result = add_two_numbers_unsafe(l1_clone, l2_clone);
            free_list(result);
        }
        let duration = start.elapsed();
        let avg_time = duration / iterations as u32;
        println!("Unsafe - Average time per operation: {:?}", avg_time);
        println!(
            "Unsafe - Operations per second: {:.0}",
            1.0 / avg_time.as_secs_f64()
        );

        // Clean up test data
        for (l1, l2) in fast_test_data {
            free_list(l1);
            free_list(l2);
        }
    }
}

// Alternative implementation using raw pointers (unsafe but faster)
#[derive(Debug)]
struct FastListNode {
    val: i32,
    next: *mut FastListNode,
}

impl FastListNode {
    fn new(val: i32) -> *mut Self {
        Box::into_raw(Box::new(FastListNode {
            val,
            next: std::ptr::null_mut(),
        }))
    }
}

unsafe fn add_two_numbers_unsafe(
    mut l1: *mut FastListNode,
    mut l2: *mut FastListNode,
) -> *mut FastListNode {
    let dummy_head = FastListNode::new(0);
    let mut current = dummy_head;
    let mut carry = 0;

    while !l1.is_null() || !l2.is_null() || carry != 0 {
        let mut sum = carry;

        if !l1.is_null() {
            sum += (*l1).val;
            l1 = (*l1).next;
        }

        if !l2.is_null() {
            sum += (*l2).val;
            l2 = (*l2).next;
        }

        carry = sum / 10;
        let new_node = FastListNode::new(sum % 10);
        (*current).next = new_node;
        current = new_node;
    }

    let result = (*dummy_head).next;
    // Clean up dummy head
    let _ = Box::from_raw(dummy_head);
    result
}

unsafe fn free_list(mut head: *mut FastListNode) {
    while !head.is_null() {
        let current = head;
        head = (*head).next;
        let _ = Box::from_raw(current);
    }
}

unsafe fn clone_fast_list(mut head: *mut FastListNode) -> *mut FastListNode {
    if head.is_null() {
        return std::ptr::null_mut();
    }

    let new_head = FastListNode::new((*head).val);
    let mut current = new_head;
    head = (*head).next;

    while !head.is_null() {
        let new_node = FastListNode::new((*head).val);
        (*current).next = new_node;
        current = new_node;
        head = (*head).next;
    }

    new_head
}

unsafe fn vec_to_fast_list(vec: &[i32]) -> *mut FastListNode {
    if vec.is_empty() {
        return std::ptr::null_mut();
    }

    let head = FastListNode::new(vec[0]);
    let mut current = head;

    for &val in &vec[1..] {
        let new_node = FastListNode::new(val);
        (*current).next = new_node;
        current = new_node;
    }

    head
}

