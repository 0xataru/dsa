package main

import (
	"fmt"
	"time"
)

// Optimal solution using sliding window with map
// Time complexity: O(n)
// Space complexity: O(min(m,n)) where m is the size of the charset
func lengthOfLongestSubstring(s string) int {
	charIndex := make(map[rune]int)
	maxLength := 0
	start := 0

	for end, char := range s {
		// If character is already seen and is within current window
		if prevIndex, exists := charIndex[char]; exists && prevIndex >= start {
			start = prevIndex + 1
		}

		// Update character's latest index
		charIndex[char] = end

		// Update maximum length
		if currentLength := end - start + 1; currentLength > maxLength {
			maxLength = currentLength
		}
	}

	return maxLength
}

// Alternative solution using sliding window with byte array for ASCII
// More space-efficient for ASCII characters
func lengthOfLongestSubstringASCII(s string) int {
	charLastIndex := make([]int, 128)
	for i := range charLastIndex {
		charLastIndex[i] = -1
	}

	maxLength := 0
	start := 0

	for end, char := range s {
		// Check if character is within ASCII range
		if char < 128 {
			lastIndex := charLastIndex[char]

			// If character is already seen and is within current window
			if lastIndex >= start {
				start = lastIndex + 1
			}

			// Update character's latest index
			charLastIndex[char] = end
		}

		// Update maximum length
		if currentLength := end - start + 1; currentLength > maxLength {
			maxLength = currentLength
		}
	}

	return maxLength
}

// Alternative solution using sliding window with set (HashSet simulation)
// Clear logic but less efficient for shrinking window
func lengthOfLongestSubstringSet(s string) int {
	charSet := make(map[rune]bool)
	maxLength := 0
	start := 0

	runes := []rune(s)
	for end := 0; end < len(runes); end++ {
		// Shrink window until no duplicates
		for charSet[runes[end]] {
			delete(charSet, runes[start])
			start++
		}

		// Add current character to set
		charSet[runes[end]] = true

		// Update maximum length
		if currentLength := end - start + 1; currentLength > maxLength {
			maxLength = currentLength
		}
	}

	return maxLength
}

func main() {
	// Test cases
	testCases := []struct {
		input    string
		expected int
	}{
		{"abcabcbb", 3},
		{"bbbbb", 1},
		{"pwwkew", 3},
		{"", 0},
		{"au", 2},
		{"dvdf", 3},
		{"abba", 2},
		{"tmmzuxt", 5},
		{"abcdef", 6},
		{"aab", 2},
	}

	fmt.Println("Testing Longest Substring Without Repeating Characters")
	fmt.Println("============================================================")

	for i, tc := range testCases {
		result1 := lengthOfLongestSubstring(tc.input)
		result2 := lengthOfLongestSubstringASCII(tc.input)
		result3 := lengthOfLongestSubstringSet(tc.input)

		fmt.Printf("Test %d: '%s'\n", i+1, tc.input)
		fmt.Printf("  Expected: %d\n", tc.expected)
		fmt.Printf("  Map:      %d %s\n", result1, getCheckMark(result1 == tc.expected))
		fmt.Printf("  ASCII:    %d %s\n", result2, getCheckMark(result2 == tc.expected))
		fmt.Printf("  Set:      %d %s\n", result3, getCheckMark(result3 == tc.expected))
		fmt.Println()
	}

	// Performance test
	fmt.Println("Performance Test:")
	fmt.Println("------------------------------")

	largeString := ""
	alphabet := "abcdefghijklmnopqrstuvwxyz"
	for i := 0; i < 1000; i++ {
		largeString += alphabet
	}

	start := time.Now()
	result := lengthOfLongestSubstring(largeString)
	duration := time.Since(start)

	fmt.Printf("Large string (26000 chars): %v\n", duration)
	fmt.Printf("Result: %d\n", result)

	// Micro-benchmark: measure only the algorithm execution
	fmt.Println("\n=== Micro-benchmark ===")

	// Pre-create test data to avoid measuring allocation in the loop
	testStrings := []string{
		"abcabcbb",
		"bbbbb",
		"pwwkew",
		"abcdefghijklmnopqrstuvwxyz",
		"aab",
		"dvdf",
		"tmmzuxt",
		"abba",
		"au",
		"",
	}

	iterations := 100000

	// Test Map implementation
	start = time.Now()
	for i := 0; i < iterations; i++ {
		testString := testStrings[i%len(testStrings)]
		_ = lengthOfLongestSubstring(testString)
	}
	duration = time.Since(start)
	avgTime := duration / time.Duration(iterations)
	fmt.Printf("Map - Average time per operation: %v\n", avgTime)
	fmt.Printf("Map - Operations per second: %.0f\n", 1.0/avgTime.Seconds())

	// Test ASCII implementation
	start = time.Now()
	for i := 0; i < iterations; i++ {
		testString := testStrings[i%len(testStrings)]
		_ = lengthOfLongestSubstringASCII(testString)
	}
	duration = time.Since(start)
	avgTime = duration / time.Duration(iterations)
	fmt.Printf("ASCII - Average time per operation: %v\n", avgTime)
	fmt.Printf("ASCII - Operations per second: %.0f\n", 1.0/avgTime.Seconds())

	// Test Set implementation
	start = time.Now()
	for i := 0; i < iterations; i++ {
		testString := testStrings[i%len(testStrings)]
		_ = lengthOfLongestSubstringSet(testString)
	}
	duration = time.Since(start)
	avgTime = duration / time.Duration(iterations)
	fmt.Printf("Set - Average time per operation: %v\n", avgTime)
	fmt.Printf("Set - Operations per second: %.0f\n", 1.0/avgTime.Seconds())

	// Memory usage test
	fmt.Println("\n=== Memory Usage Test ===")
	hugeString := ""
	charSet := "abcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()"
	for i := 0; i < 1000; i++ {
		hugeString += charSet
	}
	fmt.Printf("Huge string length: %d characters\n", len(hugeString))

	start = time.Now()
	resultMap := lengthOfLongestSubstring(hugeString)
	timeMap := time.Since(start)

	start = time.Now()
	resultASCII := lengthOfLongestSubstringASCII(hugeString)
	timeASCII := time.Since(start)

	fmt.Printf("Map result: %d (time: %v)\n", resultMap, timeMap)
	fmt.Printf("ASCII result: %d (time: %v)\n", resultASCII, timeASCII)

	// Test with different character sets
	fmt.Println("\n=== Character Set Performance ===")

	asciiString := ""
	for i := 0; i < 100; i++ {
		asciiString += "The quick brown fox jumps over the lazy dog 1234567890"
	}

	unicodeString := ""
	for i := 0; i < 100; i++ {
		unicodeString += "Привет мир! Hello world! 你好世界! こんにちは世界!"
	}

	mixedString := ""
	for i := 0; i < 200; i++ {
		mixedString += "abc123!@#АБВ你好こんにちは"
	}

	charsetTestCases := []struct {
		name string
		str  string
	}{
		{"ASCII", asciiString},
		{"Unicode", unicodeString},
		{"Mixed", mixedString},
	}

	for _, tc := range charsetTestCases {
		start = time.Now()
		result := lengthOfLongestSubstring(tc.str)
		duration := time.Since(start)

		fmt.Printf("%s (%d chars): result=%d, time=%v\n",
			tc.name, len([]rune(tc.str)), result, duration)
	}

	// Worst case scenario test
	fmt.Println("\n=== Worst Case Scenario ===")
	worstCase := ""
	for i := 0; i < 10000; i++ {
		worstCase += "a" // All same characters
	}

	bestCase := ""
	for i := 0; i < 10000; i++ {
		bestCase += string(rune('a' + i%26)) // All different (cycling through alphabet)
	}

	start = time.Now()
	worstResult := lengthOfLongestSubstring(worstCase)
	worstTime := time.Since(start)

	start = time.Now()
	bestResult := lengthOfLongestSubstring(bestCase)
	bestTime := time.Since(start)

	fmt.Printf("Worst case (all same): result=%d, time=%v\n", worstResult, worstTime)
	fmt.Printf("Best case (all different): result=%d, time=%v\n", bestResult, bestTime)

	// Unicode test
	fmt.Println("\n=== Unicode Test ===")
	unicodeString = "こんにちは世界"
	unicodeResult := lengthOfLongestSubstring(unicodeString)
	fmt.Printf("Unicode string: '%s'\n", unicodeString)
	fmt.Printf("Result: %d\n", unicodeResult)
}

func getCheckMark(condition bool) string {
	if condition {
		return "✓"
	}
	return "✗"
}
