package main

import (
	"fmt"
	"strings"
	"time"
)

// expandAroundCenter expands around center and returns palindrome length
// Time Complexity: O(n) in worst case
// Space Complexity: O(1)
func expandAroundCenter(runes []rune, left, right int) int {
	for left >= 0 && right < len(runes) && runes[left] == runes[right] {
		left--
		right++
	}
	return right - left - 1
}

// longestPalindrome finds the longest palindromic substring using Expand Around Centers
// Time Complexity: O(n²) - we expand around each possible center
// Space Complexity: O(1) - we only use constant extra space (not counting the result)
func longestPalindrome(s string) string {
	if len(s) == 0 {
		return ""
	}

	// Convert to rune slice for proper Unicode handling
	runes := []rune(s)
	n := len(runes)
	start := 0
	maxLen := 1

	for i := 0; i < n; i++ {
		// Check for odd-length palindromes (center at i)
		len1 := expandAroundCenter(runes, i, i)

		// Check for even-length palindromes (center between i and i+1)
		len2 := expandAroundCenter(runes, i, i+1)

		currentMax := max(len1, len2)

		if currentMax > maxLen {
			maxLen = currentMax
			start = i - (currentMax-1)/2
		}
	}

	// Convert back to string
	return string(runes[start : start+maxLen])
}

// longestPalindromeManacher finds the longest palindromic substring using Manacher's Algorithm
// Time Complexity: O(n) - linear time algorithm
// Space Complexity: O(n) - for preprocessed string and auxiliary arrays
func longestPalindromeManacher(s string) string {
	if len(s) == 0 {
		return ""
	}

	// Preprocess the string: "abc" -> "^#a#b#c#$"
	// ^ and $ are sentinels to avoid bounds checking
	processed := "^#"
	for _, c := range s {
		processed += string(c) + "#"
	}
	processed += "$"

	runes := []rune(processed)
	n := len(runes)
	p := make([]int, n) // p[i] = radius of palindrome centered at i
	center := 0         // center of rightmost palindrome
	right := 0          // right boundary of rightmost palindrome

	for i := 1; i < n-1; i++ {
		// Mirror of i with respect to center
		mirror := 2*center - i

		// If i is within right boundary, we can use previously computed values
		if i < right {
			p[i] = min(right-i, p[mirror])
		}

		// Try to expand palindrome centered at i
		for runes[i+p[i]+1] == runes[i-p[i]-1] {
			p[i]++
		}

		// If palindrome centered at i extends past right, update center and right
		if i+p[i] > right {
			center = i
			right = i + p[i]
		}
	}

	// Find the longest palindrome
	maxLen := 0
	centerIndex := 0
	for i := 1; i < n-1; i++ {
		if p[i] > maxLen {
			maxLen = p[i]
			centerIndex = i
		}
	}

	// Extract the palindrome from original string
	start := (centerIndex - maxLen) / 2
	originalRunes := []rune(s)
	return string(originalRunes[start : start+maxLen])
}

// max returns the maximum of two integers
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// min returns the minimum of two integers
func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

// TestCase represents a test case for the algorithm
type TestCase struct {
	input    string
	expected []string // Multiple valid answers possible
	name     string
}

func main() {
	testCases := []TestCase{
		{
			input:    "babad",
			expected: []string{"bab", "aba"},
			name:     "Example 1",
		},
		{
			input:    "cbbd",
			expected: []string{"bb"},
			name:     "Example 2",
		},
		{
			input:    "a",
			expected: []string{"a"},
			name:     "Single character",
		},
		{
			input:    "ac",
			expected: []string{"a", "c"},
			name:     "Two different characters",
		},
		{
			input:    "racecar",
			expected: []string{"racecar"},
			name:     "Full palindrome",
		},
		{
			input:    "noon",
			expected: []string{"noon"},
			name:     "Even-length palindrome",
		},
		{
			input:    "abcdcba",
			expected: []string{"abcdcba"},
			name:     "Odd-length full palindrome",
		},
		{
			input:    "bananas",
			expected: []string{"anana"},
			name:     "Embedded palindrome",
		},
	}

	fmt.Println("=== Testing Both Algorithms ===")

	for _, tc := range testCases {
		result1 := longestPalindrome(tc.input)
		result2 := longestPalindromeManacher(tc.input)

		isValid1 := contains(tc.expected, result1)
		isValid2 := contains(tc.expected, result2)

		fmt.Printf("%s: input=\"%s\"\n", tc.name, tc.input)
		fmt.Printf("  Expand Around Centers: \"%s\" %s\n", result1, getStatus(isValid1))
		fmt.Printf("  Manacher's Algorithm:  \"%s\" %s\n\n", result2, getStatus(isValid2))
	}

	// Performance comparison
	fmt.Println("=== Performance Comparison ===")

	// Test different sizes
	sizes := []int{100, 1000, 5000, 10000}

	for _, size := range sizes {
		fmt.Printf("\nTesting with size: %d\n", size)

		// Create test string with embedded palindrome
		testInput := strings.Repeat("a", size/2) + "racecar" + strings.Repeat("b", size/2)

		// Test Expand Around Centers (O(n²))
		start := time.Now()
		result1 := longestPalindrome(testInput)
		duration1 := time.Since(start)

		// Test Manacher's Algorithm (O(n))
		start = time.Now()
		result2 := longestPalindromeManacher(testInput)
		duration2 := time.Since(start)

		fmt.Printf("  Expand Around Centers (O(n²)): %8.3fms -> \"%s\"\n",
			float64(duration1.Nanoseconds())/1e6,
			truncateString(result1, 20))

		fmt.Printf("  Manacher's Algorithm  (O(n)) : %8.3fms -> \"%s\"\n",
			float64(duration2.Nanoseconds())/1e6,
			truncateString(result2, 20))

		speedup := float64(duration1.Nanoseconds()) / float64(duration2.Nanoseconds())
		if speedup > 1.0 {
			fmt.Printf("  Manacher is %.2fx faster\n", speedup)
		} else {
			fmt.Printf("  Expand Around Centers is %.2fx faster\n", 1.0/speedup)
		}
	}
}

// contains checks if a slice contains a specific string
func contains(slice []string, item string) bool {
	for _, s := range slice {
		if s == item {
			return true
		}
	}
	return false
}

// getStatus returns a checkmark or X based on boolean
func getStatus(valid bool) string {
	if valid {
		return "✓"
	}
	return "✗"
}

// truncateString truncates a string to maxLen characters
func truncateString(s string, maxLen int) string {
	if len(s) > maxLen {
		return s[:maxLen] + "..."
	}
	return s
}
