package main

import "fmt"

// limit is one past the maximum possible result length; lengths are clamped
// here so the doubling from '#' never overflows uint64 while staying large
// enough for any valid k.
const limit = uint64(1_000_000_000_000_000) + 1

// processStr finds the kth character of the final string without ever
// materializing it: a forward pass records the (clamped) length after each
// prefix, then a backward pass rewrites k into the index it occupied before
// each operation until it lands on the source letter.
func processStr(s string, k int64) byte {
	k0 := uint64(k)
	n := len(s)

	// length[i] = length of result after processing the first i characters.
	length := make([]uint64, n+1)
	for i := 0; i < n; i++ {
		switch c := s[i]; {
		case c >= 'a' && c <= 'z':
			length[i+1] = min(length[i]+1, limit)
		case c == '*':
			if length[i] > 0 {
				length[i+1] = length[i] - 1
			} else {
				length[i+1] = 0
			}
		case c == '#':
			length[i+1] = min(length[i]*2, limit)
		case c == '%':
			length[i+1] = length[i]
		}
	}

	curLen := length[n]
	if k0 >= curLen {
		return '.'
	}

	idx := k0
	for i := n - 1; i >= 0; i-- {
		switch c := s[i]; {
		case c >= 'a' && c <= 'z':
			prevLen := length[i]
			// The appended letter sits at the last position.
			if idx == prevLen {
				return c
			}
			curLen = prevLen
		case c == '*':
			// A removed character left no trace; idx is unaffected.
			curLen++
		case c == '#':
			prevLen := length[i]
			// The second half mirrors the first, fold idx into the original.
			if prevLen > 0 {
				idx %= prevLen
			}
			curLen = prevLen
		case c == '%':
			// Reversal maps index idx to curLen - 1 - idx.
			if curLen > 0 {
				idx = curLen - 1 - idx
			}
		}
	}

	return '.'
}

func main() {
	testCases := []struct {
		s        string
		k        int64
		expected byte
		name     string
	}{
		{"a#b%*", 1, 'a', "Example 1"},
		{"cd%#*#", 3, 'd', "Example 2"},
		{"z*#", 0, '.', "Example 3 (out of bounds)"},
		{"abc", 0, 'a', "No special chars, first"},
		{"abc", 2, 'c', "No special chars, last"},
		{"ab#%", 0, 'b', "Duplicate then reverse"},
		{"ab#%", 3, 'a', "Duplicate then reverse, last"},
		{"a%", 0, 'a', "Reverse single char"},
		{"ab###", 15, 'b', "Repeated duplication"},
	}

	fmt.Println("=== Process String with Special Operations II ===")

	for _, tc := range testCases {
		result := processStr(tc.s, tc.k)
		status := "✓"
		if result != tc.expected {
			status = "✗"
		}
		fmt.Printf("%s: %q, k=%d -> %q %s (expected %q)\n",
			tc.name, tc.s, tc.k, result, status, tc.expected)
	}
}
