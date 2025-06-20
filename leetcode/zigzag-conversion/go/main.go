package main

import (
	"fmt"
	"strings"
	"time"
)

// convert transforms string into zigzag pattern and reads line by line
// Time Complexity: O(n) - we visit each character exactly once
// Space Complexity: O(n) - we store characters in rows
func convert(s string, numRows int) string {
	// Edge case: if numRows is 1 or string is shorter than numRows
	if numRows == 1 || len(s) <= numRows {
		return s
	}

	// Create a slice of strings for each row
	rows := make([]strings.Builder, numRows)
	currentRow := 0
	goingDown := false

	// Traverse each character in the string
	for _, char := range s {
		// Add character to current row
		rows[currentRow].WriteRune(char)

		// Change direction when reaching top or bottom
		if currentRow == 0 || currentRow == numRows-1 {
			goingDown = !goingDown
		}

		// Move to next row based on direction
		if goingDown {
			currentRow++
		} else {
			currentRow--
		}
	}

	// Concatenate all rows
	var result strings.Builder
	for i := 0; i < numRows; i++ {
		result.WriteString(rows[i].String())
	}

	return result.String()
}

// Alternative mathematical approach - more space efficient
// Time Complexity: O(n) - we visit each character position once
// Space Complexity: O(1) - only using constant extra space (excluding result)
func convertMath(s string, numRows int) string {
	if numRows == 1 || len(s) <= numRows {
		return s
	}

	var result strings.Builder
	n := len(s)
	cycleLen := 2*numRows - 2 // Length of one complete zigzag cycle

	for i := 0; i < numRows; i++ {
		for j := 0; j+i < n; j += cycleLen {
			// Add character from current position
			result.WriteByte(s[j+i])

			// Add middle character (if not first or last row)
			if i != 0 && i != numRows-1 && j+cycleLen-i < n {
				result.WriteByte(s[j+cycleLen-i])
			}
		}
	}

	return result.String()
}

func main() {
	// Test cases
	testCases := []struct {
		s       string
		numRows int
		name    string
	}{
		{"PAYPALISHIRING", 3, "Example 1"},
		{"PAYPALISHIRING", 4, "Example 2"},
		{"A", 1, "Example 3"},
		{"AB", 1, "Single row"},
		{"ABCDEFGHIJKLMNOP", 4, "Longer string"},
		{"", 3, "Empty string"},
		{"ABCD", 2, "Simple pattern"},
		{"ABCDEFGHIJ", 5, "Multiple cycles"},
	}

	fmt.Println("=== ZigZag Conversion Solutions ===")

	for _, tc := range testCases {
		result1 := convert(tc.s, tc.numRows)
		result2 := convertMath(tc.s, tc.numRows)

		fmt.Printf("%s:\n", tc.name)
		fmt.Printf("  Input: s = \"%s\", numRows = %d\n", tc.s, tc.numRows)
		fmt.Printf("  Output (Row-by-row): \"%s\"\n", result1)
		fmt.Printf("  Output (Mathematical): \"%s\"\n", result2)

		// Verify both solutions match
		if result1 == result2 {
			fmt.Printf("  ✓ Both solutions match\n")
		} else {
			fmt.Printf("  ✗ Solutions differ!\n")
		}

		// Show pattern visualization for small strings
		if len(tc.s) <= 20 && tc.numRows > 1 {
			visualizePattern(tc.s, tc.numRows)
		}
		fmt.Println()
	}

	// Performance comparison
	fmt.Println("=== Performance Test ===")
	largeString := strings.Repeat("ABCDEFGHIJ", 1000) // 10,000 characters
	numRows := 7

	// Test row-by-row approach
	start := now()
	result1 := convert(largeString, numRows)
	duration1 := since(start)

	// Test mathematical approach
	start = now()
	result2 := convertMath(largeString, numRows)
	duration2 := since(start)

	fmt.Printf("Large string (10,000 chars), numRows = %d:\n", numRows)
	fmt.Printf("  Row-by-row approach: %v\n", duration1)
	fmt.Printf("  Mathematical approach: %v\n", duration2)
	fmt.Printf("  Results match: %v\n", result1 == result2)
	fmt.Printf("  Mathematical is %.2fx faster\n", float64(duration1.Nanoseconds())/float64(duration2.Nanoseconds()))
}

// Helper function to visualize the zigzag pattern
func visualizePattern(s string, numRows int) {
	if numRows == 1 {
		return
	}

	fmt.Printf("  Pattern visualization:\n")
	rows := make([][]rune, numRows)
	for i := range rows {
		rows[i] = make([]rune, len(s))
		for j := range rows[i] {
			rows[i][j] = ' '
		}
	}

	currentRow := 0
	goingDown := false
	col := 0

	for _, char := range s {
		rows[currentRow][col] = char

		if currentRow == 0 || currentRow == numRows-1 {
			goingDown = !goingDown
		}

		if goingDown {
			currentRow++
		} else {
			currentRow--
		}
		col++
	}

	// Print the pattern
	for i := 0; i < numRows; i++ {
		fmt.Print("    ")
		for j := 0; j < len(s); j++ {
			if rows[i][j] != ' ' {
				fmt.Printf("%c", rows[i][j])
			} else {
				fmt.Print(" ")
			}
		}
		fmt.Println()
	}
}

// Helper functions for timing (simplified versions)
func now() time.Time {
	return time.Now()
}

func since(start time.Time) time.Duration {
	return time.Since(start)
}
