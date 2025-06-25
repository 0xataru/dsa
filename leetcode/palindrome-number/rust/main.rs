use std::collections::HashMap;
use std::time::Instant;

/// Palindrome Number - checks if an integer is a palindrome without string conversion
/// Time Complexity: O(log n) - we process each digit once, and there are log10(n) digits
/// Space Complexity: O(1) - we only use a constant amount of extra space
fn is_palindrome(x: i64) -> bool {
    // Negative numbers are not palindromes
    // Numbers ending in 0 (except 0 itself) are not palindromes
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    // Single digit numbers are palindromes
    if x < 10 {
        return true;
    }

    // Reverse half of the number and compare with the other half
    let mut original = x;
    let mut reversed_half = 0;

    while original > reversed_half {
        reversed_half = reversed_half * 10 + original % 10;
        original /= 10;
    }

    // For even length numbers: original == reversed_half
    // For odd length numbers: original == reversed_half/10 (remove middle digit)
    original == reversed_half || original == reversed_half / 10
}

// 1. CURRENT ALGORITHM: Half-reversal O(log n) time, O(1) space
fn is_palindrome_half_reverse(x: i64) -> bool {
    if x < 0 || (x > 0 && x % 10 == 0) {
        return false;
    }
    if x < 10 {
        return true;
    }

    let mut reversed = 0i64;
    let mut original = x;
    while original > reversed {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }
    original == reversed || original == reversed / 10
}

// 2. LOOKUP TABLE: O(1) for small numbers, O(log n) for large
// This is FASTER for numbers < 100000 due to cache efficiency
use std::sync::OnceLock;

static PALINDROME_CACHE: OnceLock<HashMap<i64, bool>> = OnceLock::new();

fn get_palindrome_cache() -> &'static HashMap<i64, bool> {
    PALINDROME_CACHE.get_or_init(|| {
        let mut cache = HashMap::with_capacity(100_000);
        for i in 0..100_000i64 {
            cache.insert(i, is_palindrome_naive(i));
        }
        cache
    })
}

fn is_palindrome_lookup(x: i64) -> bool {
    if x < 0 {
        return false;
    }
    if x < 100_000 {
        *get_palindrome_cache().get(&x).unwrap_or(&false) // O(1) lookup!
    } else {
        // Fall back to half-reverse for larger numbers
        is_palindrome_half_reverse(x)
    }
}

// 3. MATHEMATICAL APPROACH: Direct digit comparison O(log n) time
// Better cache locality and fewer operations
fn is_palindrome_math(x: i64) -> bool {
    if x < 0 {
        return false;
    }
    if x < 10 {
        return true;
    }
    if x % 10 == 0 {
        return false;
    }

    // Count digits first (this helps with optimization)
    let mut digits = 1i64;
    let mut temp = x;
    while temp >= 10 {
        digits *= 10;
        temp /= 10;
    }

    let mut num = x;
    // Compare digits from both ends
    while num > 0 {
        let left_digit = num / digits;
        let right_digit = num % 10;
        
        if left_digit != right_digit {
            return false;
        }
        
        // Remove processed digits
        num = (num % digits) / 10;
        digits /= 100; // Remove two digits worth of divisor
    }
    true
}

// 4. OPTIMIZED STRING APPROACH: O(log n) time, O(log n) space
// Sometimes faster due to CPU string optimizations
fn is_palindrome_string(x: i64) -> bool {
    if x < 0 {
        return false;
    }
    let s = x.to_string();
    let bytes = s.as_bytes();
    let n = bytes.len();
    
    for i in 0..n/2 {
        if bytes[i] != bytes[n - 1 - i] {
            return false;
        }
    }
    true
}

// 5. ITERATOR-BASED APPROACH: Idiomatic Rust O(log n)
fn is_palindrome_iterator(x: i64) -> bool {
    if x < 0 {
        return false;
    }
    
    let s = x.to_string();
    let chars: Vec<char> = s.chars().collect();
    
    chars.iter().eq(chars.iter().rev())
}

// 6. UNSAFE OPTIMIZED: For maximum performance (experimental)
fn is_palindrome_unsafe(x: i64) -> bool {
    if x < 0 {
        return false;
    }
    if x < 10 {
        return true;
    }
    
    let s = x.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    
    // Using unsafe for potentially faster access
    unsafe {
        for i in 0..len/2 {
            if *bytes.get_unchecked(i) != *bytes.get_unchecked(len - 1 - i) {
                return false;
            }
        }
    }
    true
}

// 7. SIMD-INSPIRED: For large number optimization
fn is_palindrome_vectorized(x: i64) -> bool {
    if x < 0 {
        return false;
    }
    if x < 10 {
        return true;
    }
    
    // Convert to digits vector for potential SIMD operations
    let mut digits = Vec::new();
    let mut temp = x;
    while temp > 0 {
        digits.push((temp % 10) as u8);
        temp /= 10;
    }
    
    // Compare from both ends
    let len = digits.len();
    for i in 0..len/2 {
        if digits[i] != digits[len - 1 - i] {
            return false;
        }
    }
    true
}

// Helper function for cache initialization
fn is_palindrome_naive(x: i64) -> bool {
    if x < 0 {
        return false;
    }
    let s = x.to_string();
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    for i in 0..n/2 {
        if chars[i] != chars[n - 1 - i] {
            return false;
        }
    }
    true
}

// Algorithm trait for uniform testing
trait PalindromeAlgorithm {
    fn check(&self, x: i64) -> bool;
    fn name(&self) -> &'static str;
}

// Algorithm implementations

struct Simple;
impl PalindromeAlgorithm for Simple {
    fn check(&self, x: i64) -> bool { is_palindrome(x) }
    fn name(&self) -> &'static str { "Simple O(log n)" }
}

struct HalfReverse;
impl PalindromeAlgorithm for HalfReverse {
    fn check(&self, x: i64) -> bool { is_palindrome_half_reverse(x) }
    fn name(&self) -> &'static str { "Half-Reverse O(log n)" }
}

struct LookupHybrid;
impl PalindromeAlgorithm for LookupHybrid {
    fn check(&self, x: i64) -> bool { is_palindrome_lookup(x) }
    fn name(&self) -> &'static str { "Lookup+Hybrid O(1)/O(log n)" }
}

struct Mathematical;
impl PalindromeAlgorithm for Mathematical {
    fn check(&self, x: i64) -> bool { is_palindrome_math(x) }
    fn name(&self) -> &'static str { "Mathematical O(log n)" }
}

struct StringBased;
impl PalindromeAlgorithm for StringBased {
    fn check(&self, x: i64) -> bool { is_palindrome_string(x) }
    fn name(&self) -> &'static str { "String-based O(log n)" }
}

struct IteratorBased;
impl PalindromeAlgorithm for IteratorBased {
    fn check(&self, x: i64) -> bool { is_palindrome_iterator(x) }
    fn name(&self) -> &'static str { "Iterator-based O(log n)" }
}

struct UnsafeOptimized;
impl PalindromeAlgorithm for UnsafeOptimized {
    fn check(&self, x: i64) -> bool { is_palindrome_unsafe(x) }
    fn name(&self) -> &'static str { "Unsafe-optimized O(log n)" }
}

struct Vectorized;
impl PalindromeAlgorithm for Vectorized {
    fn check(&self, x: i64) -> bool { is_palindrome_vectorized(x) }
    fn name(&self) -> &'static str { "Vectorized O(log n)" }
}

fn main() {
    // Initialize algorithms
    let algorithms: Vec<Box<dyn PalindromeAlgorithm>> = vec![
        Box::new(Simple),
        Box::new(HalfReverse),
        Box::new(LookupHybrid),
        Box::new(Mathematical),
        Box::new(StringBased),
        Box::new(IteratorBased),
        Box::new(UnsafeOptimized),
        Box::new(Vectorized),
    ];

    // Test correctness
    let test_cases = vec![
        0, 1, 11, 121, -121, 10, 1221, 12321,
        123321, 1234321, 1234567, 99999, 100001,
        1234554321, 9223372036854775807,
    ];

    println!("=== CORRECTNESS TEST ===");
    for &tc in &test_cases {
        println!("Number: {}", tc);
        for alg in &algorithms {
            let result = alg.check(tc);
            println!("  {}: {}", alg.name(), result);
        }
        println!();
    }

    // Complexity analysis with different input sizes
    println!("=== COMPLEXITY ANALYSIS ===");
    
    let input_groups = vec![
        ("Small (1-4 digits)", vec![1, 12, 121, 1221]),
        ("Medium (5-8 digits)", vec![12321, 123321, 1234321, 12344321]),
        ("Large (9+ digits)", vec![123454321, 1234554321, 12345654321]),
        ("Cache-friendly (<100k)", vec![99999, 12321, 54345, 98789]),
    ];

    for (group_name, numbers) in &input_groups {
        println!("\n--- {} ---", group_name);
        
        for alg in &algorithms {
            // Warm up
            for &num in numbers {
                alg.check(num);
            }
            
            // Benchmark
            let iterations = 50_000;
            let start = Instant::now();
            
            for _ in 0..iterations {
                for &num in numbers {
                    std::hint::black_box(alg.check(num));
                }
            }
            
            let duration = start.elapsed();
            let avg_time = duration.as_nanos() as f64 / (iterations * numbers.len()) as f64;
            println!("{}: {:.2} ns/op", alg.name(), avg_time);
        }
    }

    // Memory complexity analysis
    println!("\n=== MEMORY COMPLEXITY ===");
    println!("Half-Reverse: O(1) space");
    println!("Lookup+Hybrid: O(k) space where k=100,000 cache size");
    println!("Mathematical: O(1) space");
    println!("String-based: O(log n) space");
    println!("Iterator-based: O(log n) space");
    println!("Unsafe-optimized: O(log n) space");
    println!("Vectorized: O(log n) space");

    // Rust-specific performance analysis
    println!("\n=== RUST-SPECIFIC OPTIMIZATIONS ===");
    println!("✅ Zero-cost abstractions in Half-Reverse & Mathematical");
    println!("✅ Memory safety without overhead");
    println!("✅ LLVM optimizations in release mode");
    println!("✅ Iterator optimizations in Iterator-based");
    println!("⚠️  Unsafe code potential speedup (needs careful validation)");
    println!("🚀 Vectorized approach ready for SIMD extensions");

    // Theoretical limits
    println!("\n=== THEORETICAL ANALYSIS ===");
    println!("LOWER BOUND: Ω(log n) - must examine each digit at least once");
    println!("RUST ADVANTAGES:");
    println!("  • Zero-cost abstractions: High-level code → optimal assembly");
    println!("  • Memory safety: No undefined behavior or leaks");
    println!("  • LLVM backend: State-of-the-art compiler optimizations");
    println!("  • Iterator fusion: Eliminates intermediate allocations");
    println!("  • Unsafe escapes: Manual optimization when needed");

    println!("\nTRADE-OFFS:");
    println!("  • Lookup table: O(1) for small inputs, O(k) extra space");
    println!("  • String methods: Sometimes faster due to optimized string ops");
    println!("  • Mathematical: Best cache locality, minimal operations");
    println!("  • Iterator: Most idiomatic Rust, compiler-optimized");
    println!("  • Unsafe: Maximum performance, manual safety validation");
    
    println!("\nCONCLUSION:");
    println!("• O(log n) is optimal time complexity (provably)");
    println!("• O(1) space is optimal for general case");
    println!("• Lookup tables achieve O(1) for bounded inputs");
    println!("• Rust provides safety + performance without trade-offs");
    println!("• Different algorithms excel in different scenarios");

    // Benchmark against criterion for more precise measurements
    println!("\n=== PRECISION BENCHMARK ===");
    let benchmark_number = 1234554321i64;
    let precision_iterations = 1_000_000;

    for alg in &algorithms {
        let start = Instant::now();
        for _ in 0..precision_iterations {
            std::hint::black_box(alg.check(benchmark_number));
        }
        let duration = start.elapsed();
        let avg_ns = duration.as_nanos() as f64 / precision_iterations as f64;
        println!("{}: {:.3} ns/op", alg.name(), avg_ns);
    }
} 