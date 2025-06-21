package main

import (
	"fmt"
	"math"
	"time"
)

// reverse reverses digits of a 32-bit signed integer
// Time Complexity: O(log n) - where n is the input number (number of digits)
// Space Complexity: O(1) - only using constant extra space
func reverse(x int) int {
	result := 0

	// Constants for 32-bit signed integer bounds
	const intMax = math.MaxInt32 // 2147483647
	const intMin = math.MinInt32 // -2147483648

	for x != 0 {
		digit := x % 10
		x /= 10

		// Check for overflow before multiplying and adding
		// For positive overflow: result > (intMax - digit) / 10
		// For negative overflow: result < (intMin - digit) / 10
		if result > intMax/10 || (result == intMax/10 && digit > 7) {
			return 0
		}
		if result < intMin/10 || (result == intMin/10 && digit < -8) {
			return 0
		}

		result = result*10 + digit
	}

	return result
}

type TestCase struct {
	input    int
	expected int
	name     string
}

func main() {
	testCases := []TestCase{
		{123, 321, "Example 1: Positive number"},
		{-123, -321, "Example 2: Negative number"},
		{120, 21, "Example 3: Trailing zeros"},
		{0, 0, "Zero"},
		{1534236469, 0, "Overflow case"},
		{-2147483648, 0, "INT_MIN overflow"},
		{1463847412, 2147483641, "Near overflow boundary"},
		{-1463847412, -2147483641, "Near negative overflow boundary"},
	}

	fmt.Println("=== Reverse Integer Test Cases ===")

	for _, tc := range testCases {
		result := reverse(tc.input)
		status := "✓"
		if result != tc.expected {
			status = "✗"
		}

		fmt.Printf("%s: input=%d, expected=%d, got=%d %s\n",
			tc.name, tc.input, tc.expected, result, status)

		if result != tc.expected {
			fmt.Printf("  ERROR: Expected %d, but got %d\n", tc.expected, result)
		}
	}

	// Performance test
	fmt.Println("\n=== Performance Test ===")
	performanceCases := []int{1000000000, -1000000000, 123456789, -987654321}

	for _, testCase := range performanceCases {
		start := time.Now()
		result := reverse(testCase)
		duration := time.Since(start)

		fmt.Printf("Input: %d, Output: %d, Time: %v\n", testCase, result, duration)
	}

	// Edge case analysis
	fmt.Println("\n=== Edge Case Analysis ===")
	fmt.Printf("math.MaxInt32 = %d\n", math.MaxInt32)
	fmt.Printf("math.MinInt32 = %d\n", math.MinInt32)
	fmt.Printf("Reverse of MaxInt32 should overflow: %d\n", reverse(math.MaxInt32))
	fmt.Printf("Reverse of MinInt32 should overflow: %d\n", reverse(math.MinInt32))
}
