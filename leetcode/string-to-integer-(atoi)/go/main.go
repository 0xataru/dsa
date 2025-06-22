package main

import (
	"fmt"
	"math"
	"time"
)

// myAtoi converts a string to a 32-bit signed integer
// Time Complexity: O(n) - where n is the length of the string
// Space Complexity: O(1) - only using constant extra space
func myAtoi(s string) int {
	i := 0
	n := len(s)

	// Step 1: Skip leading whitespace
	for i < n && s[i] == ' ' {
		i++
	}

	// Step 2: Handle sign
	sign := 1
	if i < n && (s[i] == '+' || s[i] == '-') {
		if s[i] == '-' {
			sign = -1
		}
		i++
	}

	// Step 3: Convert digits with overflow detection
	result := int64(0)
	const intMax = math.MaxInt32 // 2147483647
	const intMin = math.MinInt32 // -2147483648

	for i < n && s[i] >= '0' && s[i] <= '9' {
		digit := int64(s[i] - '0')

		// Check for overflow before adding digit
		if result > (intMax-digit)/10 {
			if sign == 1 {
				return intMax
			} else {
				return intMin
			}
		}

		result = result*10 + digit
		i++
	}

	// Apply sign and ensure within 32-bit range
	finalResult := int64(sign) * result

	// Clamp to 32-bit range
	if finalResult > intMax {
		return intMax
	}
	if finalResult < intMin {
		return intMin
	}

	return int(finalResult)
}

type TestCase struct {
	input    string
	expected int
	name     string
}

func main() {
	testCases := []TestCase{
		{"42", 42, "Example 1: Simple positive"},
		{"   -042", -42, "Example 2: Leading whitespace and zeros"},
		{"1337c0d3", 1337, "Example 3: Stops at non-digit"},
		{"0-1", 0, "Example 4: Stops at non-digit after zero"},
		{"words and 987", 0, "Example 5: Starts with non-digit"},
		{"", 0, "Empty string"},
		{"   ", 0, "Only whitespace"},
		{"+", 0, "Only sign"},
		{"-", 0, "Only negative sign"},
		{"2147483647", 2147483647, "INT_MAX"},
		{"2147483648", 2147483647, "Overflow to INT_MAX"},
		{"-2147483648", -2147483648, "INT_MIN"},
		{"-2147483649", -2147483648, "Underflow to INT_MIN"},
		{"9223372036854775808", 2147483647, "Very large overflow"},
		{"  0000000000012345678", 12345678, "Leading zeros"},
		{"+-12", 0, "Invalid double sign"},
		{"   +0 123", 0, "Space after digits"},
	}

	fmt.Println("=== String to Integer (atoi) Test Cases ===")

	for _, tc := range testCases {
		result := myAtoi(tc.input)
		status := "✓"
		if result != tc.expected {
			status = "✗"
		}

		fmt.Printf("%s: input=\"%s\" expected=%d, got=%d %s\n",
			tc.name, tc.input, tc.expected, result, status)

		if result != tc.expected {
			fmt.Printf("  ERROR: Expected %d, but got %d\n", tc.expected, result)
		}
	}

	// Performance test
	fmt.Println("\n=== Performance Test ===")
	performanceCases := []string{
		"1234567890",
		"   -987654321",
		"2147483647",
		"-2147483648",
		"99999999999999999999",
	}

	for _, testCase := range performanceCases {
		start := time.Now()
		result := myAtoi(testCase)
		duration := time.Since(start)

		fmt.Printf("Input: \"%s\", Output: %d, Time: %v\n", testCase, result, duration)
	}

	// Edge case analysis
	fmt.Println("\n=== Edge Case Analysis ===")
	fmt.Printf("math.MaxInt32 = %d\n", math.MaxInt32)
	fmt.Printf("math.MinInt32 = %d\n", math.MinInt32)

	edgeCases := []struct {
		desc  string
		input string
	}{
		{"4193 with words", "4193 with words"},
		{"-91283472332", "-91283472332"},
		{"91283472332", "91283472332"},
		{"20000000000000000000", "20000000000000000000"},
	}

	for _, ec := range edgeCases {
		result := myAtoi(ec.input)
		fmt.Printf("%s: \"%s\" -> %d\n", ec.desc, ec.input, result)
	}
}
