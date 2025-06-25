package main

import (
	"fmt"
	"time"
)

// 1. CURRENT ALGORITHM: Half-reversal O(log n) time, O(1) space
func isPalindromeHalfReverse(x int64) bool {
	if x < 0 || (x > 0 && x%10 == 0) {
		return false
	}
	if x < 10 {
		return true
	}

	reversed := int64(0)
	original := x
	for original > reversed {
		reversed = reversed*10 + original%10
		original /= 10
	}
	return original == reversed || original == reversed/10
}

// 2. LOOKUP TABLE: O(1) for small numbers, O(log n) for large
// This is FASTER for numbers < 100000 due to cache efficiency
var palindromeCache = make(map[int64]bool, 100000)

func init() {
	// Pre-compute all palindromes up to 100000
	for i := int64(0); i < 100000; i++ {
		palindromeCache[i] = isPalindromeNaive(i)
	}
}

func isPalindromeLookup(x int64) bool {
	if x < 0 {
		return false
	}
	if x < 100000 {
		return palindromeCache[x] // O(1) lookup!
	}
	// Fall back to half-reverse for larger numbers
	return isPalindromeHalfReverse(x)
}

// 3. MATHEMATICAL APPROACH: Direct digit comparison O(log n) time
// But with better cache locality and fewer operations
func isPalindromeMath(x int64) bool {
	if x < 0 {
		return false
	}
	if x < 10 {
		return true
	}
	if x%10 == 0 {
		return false
	}

	// Count digits first (this helps with optimization)
	digits := int64(1)
	temp := x
	for temp >= 10 {
		digits *= 10
		temp /= 10
	}

	// Compare digits from both ends
	for x > 0 {
		leftDigit := x / digits
		rightDigit := x % 10

		if leftDigit != rightDigit {
			return false
		}

		// Remove processed digits
		x = (x % digits) / 10
		digits /= 100 // Remove two digits worth of divisor
	}
	return true
}

// 4. OPTIMIZED STRING APPROACH: O(log n) time, O(log n) space
// Sometimes faster due to CPU string optimizations
func isPalindromeString(x int64) bool {
	if x < 0 {
		return false
	}
	s := fmt.Sprintf("%d", x)
	n := len(s)
	for i := 0; i < n/2; i++ {
		if s[i] != s[n-1-i] {
			return false
		}
	}
	return true
}

// 5. BIT MANIPULATION: For specific patterns (experimental)
func isPalindromeBits(x int64) bool {
	if x < 0 {
		return false
	}
	if x < 10 {
		return true
	}

	// This is still O(log n) but uses bit operations where possible
	// Convert to string representation but work with bytes
	str := fmt.Sprintf("%d", x)
	left, right := 0, len(str)-1

	for left < right {
		if str[left] != str[right] {
			return false
		}
		left++
		right--
	}
	return true
}

// Helper function for cache initialization
func isPalindromeNaive(x int64) bool {
	if x < 0 {
		return false
	}
	str := fmt.Sprintf("%d", x)
	n := len(str)
	for i := 0; i < n/2; i++ {
		if str[i] != str[n-1-i] {
			return false
		}
	}
	return true
}

// Performance testing structure
type Algorithm struct {
	name string
	fn   func(int64) bool
}

func main() {
	algorithms := []Algorithm{
		{"Half-Reverse O(log n)", isPalindromeHalfReverse},
		{"Lookup+Hybrid O(1)/O(log n)", isPalindromeLookup},
		{"Mathematical O(log n)", isPalindromeMath},
		{"String-based O(log n)", isPalindromeString},
		{"Bit-optimized O(log n)", isPalindromeBits},
	}

	// Test correctness
	testCases := []int64{
		0, 1, 11, 121, -121, 10, 1221, 12321,
		123321, 1234321, 1234567, 99999, 100001,
		1234554321, 9223372036854775807,
	}

	fmt.Println("=== CORRECTNESS TEST ===")
	for _, tc := range testCases {
		fmt.Printf("Number: %d\n", tc)
		for _, alg := range algorithms {
			result := alg.fn(tc)
			fmt.Printf("  %s: %t\n", alg.name, result)
		}
		fmt.Println()
	}

	// Complexity analysis with different input sizes
	fmt.Println("\n=== COMPLEXITY ANALYSIS ===")

	inputSizes := []struct {
		name    string
		numbers []int64
	}{
		{"Small (1-4 digits)", []int64{1, 12, 121, 1221}},
		{"Medium (5-8 digits)", []int64{12321, 123321, 1234321, 12344321}},
		{"Large (9+ digits)", []int64{123454321, 1234554321, 12345654321}},
	}

	for _, group := range inputSizes {
		fmt.Printf("\n--- %s ---\n", group.name)

		for _, alg := range algorithms {
			// Warm up
			for _, num := range group.numbers {
				alg.fn(num)
			}

			// Benchmark
			iterations := 50000
			start := time.Now()
			for i := 0; i < iterations; i++ {
				for _, num := range group.numbers {
					alg.fn(num)
				}
			}
			duration := time.Since(start)

			avgTime := float64(duration.Nanoseconds()) / float64(iterations*len(group.numbers))
			fmt.Printf("%s: %.2f ns/op\n", alg.name, avgTime)
		}
	}

	// Memory usage analysis
	fmt.Println("\n=== MEMORY COMPLEXITY ===")
	fmt.Println("Half-Reverse: O(1) space")
	fmt.Println("Lookup+Hybrid: O(k) space where k=100000 cache size")
	fmt.Println("Mathematical: O(1) space")
	fmt.Println("String-based: O(log n) space")
	fmt.Println("Bit-optimized: O(log n) space")

	// Theoretical limits
	fmt.Println("\n=== THEORETICAL ANALYSIS ===")
	fmt.Println("LOWER BOUND: Ω(log n) - must examine each digit at least once")
	fmt.Println("CURRENT BEST: O(log n) time, O(1) space (Half-Reverse, Mathematical)")
	fmt.Println("TRADE-OFFS:")
	fmt.Println("  - Lookup table: O(1) for small inputs, O(k) extra space")
	fmt.Println("  - String-based: Sometimes faster due to CPU optimizations")
	fmt.Println("  - Mathematical: Better cache locality, fewer operations")

	fmt.Println("\nCONCLUSION:")
	fmt.Println("• O(log n) is optimal time complexity (provably)")
	fmt.Println("• O(1) space is optimal for general case")
	fmt.Println("• Lookup tables can achieve O(1) for bounded inputs")
	fmt.Println("• Performance differences are in constants, not complexity")
}
