package main

import "fmt"

// maxTotalValue returns the maximum sum of k subarray (max - min) values.
// Time: O(n), Space: O(1)
func maxTotalValue(nums []int, k int) int64 {
	minVal, maxVal := nums[0], nums[0]
	for _, num := range nums[1:] {
		if num < minVal {
			minVal = num
		}
		if num > maxVal {
			maxVal = num
		}
	}

	return int64(maxVal-minVal) * int64(k)
}

func main() {
	testCases := []struct {
		nums     []int
		k        int
		expected int64
		name     string
	}{
		{[]int{1, 3, 2}, 2, 4, "Example 1"},
		{[]int{4, 2, 5, 1}, 3, 12, "Example 2"},
		{[]int{7}, 5, 0, "Single element"},
		{[]int{1, 1, 1, 1}, 10, 0, "All equal"},
		{[]int{0, 100}, 1, 100, "Two elements"},
	}

	fmt.Println("=== Maximum Total Subarray Value I ===")

	for _, tc := range testCases {
		result := maxTotalValue(tc.nums, tc.k)
		status := "✓"
		if result != tc.expected {
			status = "✗"
		}

		fmt.Printf(
			"%s: nums=%v, k=%d -> %d %s (expected %d)\n",
			tc.name, tc.nums, tc.k, result, status, tc.expected,
		)
	}
}
