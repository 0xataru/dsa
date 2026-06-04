package main

import (
	"fmt"
	"strconv"
)

// wavinessOfNumber returns peak/valley count for one number (inner digits only).
func wavinessOfNumber(num int) int {
	s := strconv.Itoa(num)
	if len(s) < 3 {
		return 0
	}

	count := 0
	for i := 1; i < len(s)-1; i++ {
		prev := int(s[i-1] - '0')
		curr := int(s[i] - '0')
		next := int(s[i+1] - '0')

		if (curr > prev && curr > next) || (curr < prev && curr < next) {
			count++
		}
	}

	return count
}

// totalWaviness sums waviness for all numbers in [num1, num2].
// Time: O((num2 - num1 + 1) * d), Space: O(d) per number
func totalWaviness(num1, num2 int) int {
	waviness := 0
	for num := num1; num <= num2; num++ {
		waviness += wavinessOfNumber(num)
	}
	return waviness
}

func main() {
	testCases := []struct {
		num1, num2 int
		expected   int
		name       string
	}{
		{120, 130, 3, "Example 1"},
		{198, 202, 3, "Example 2"},
		{4848, 4848, 2, "Example 3"},
		{1, 99, 0, "All numbers have fewer than 3 digits"},
		{100, 100, 0, "Three digits, no peak or valley"},
	}

	fmt.Println("=== Total Waviness Test Cases ===")

	for _, tc := range testCases {
		result := totalWaviness(tc.num1, tc.num2)
		status := "✓"
		if result != tc.expected {
			status = "✗"
		}

		fmt.Printf(
			"%s: num1=%d, num2=%d -> expected=%d, got=%d %s\n",
			tc.name, tc.num1, tc.num2, tc.expected, result, status,
		)
	}
}
