package main

import "fmt"

// processStr simulates each operation on a mutable byte slice.
func processStr(s string) string {
	result := []byte{}

	for _, ch := range s {
		switch ch {
		case '*':
			if len(result) > 0 {
				result = result[:len(result)-1]
			}
		case '#':
			result = append(result, result...)
		case '%':
			for i, j := 0, len(result)-1; i < j; i, j = i+1, j-1 {
				result[i], result[j] = result[j], result[i]
			}
		default:
			result = append(result, byte(ch))
		}
	}

	return string(result)
}

func main() {
	testCases := []struct {
		s        string
		expected string
		name     string
	}{
		{"a#b%*", "ba", "Example 1"},
		{"z*#", "", "Example 2"},
		{"abc", "abc", "No special chars"},
		{"***", "", "Pop on empty"},
		{"a%", "a", "Reverse single char"},
		{"ab#%", "baba", "Duplicate then reverse"},
	}

	fmt.Println("=== Process String with Special Operations I ===")

	for _, tc := range testCases {
		result := processStr(tc.s)
		status := "✓"
		if result != tc.expected {
			status = "✗"
		}
		fmt.Printf("%s: %q -> %q %s (expected %q)\n", tc.name, tc.s, result, status, tc.expected)
	}
}