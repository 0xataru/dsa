package main

import "fmt"

// pivotArray partitions nums: less than pivot, equal, greater (stable within groups).
// Time: O(n), Space: O(n)
func pivotArray(nums []int, pivot int) []int {
	left := make([]int, 0, len(nums))
	equal := make([]int, 0)
	right := make([]int, 0)

	for _, num := range nums {
		switch {
		case num < pivot:
			left = append(left, num)
		case num == pivot:
			equal = append(equal, num)
		default:
			right = append(right, num)
		}
	}

	result := make([]int, 0, len(nums))
	result = append(result, left...)
	result = append(result, equal...)
	result = append(result, right...)
	return result
}

func main() {
	testCases := []struct {
		nums     []int
		pivot    int
		expected []int
		name     string
	}{
		{[]int{9, 12, 5, 10, 14, 3, 10}, 10, []int{9, 5, 3, 10, 10, 12, 14}, "Example 1"},
		{[]int{-3, 4, 3, 2}, 2, []int{-3, 2, 4, 3}, "Example 2"},
		{[]int{2, 2, 2}, 2, []int{2, 2, 2}, "All equal to pivot"},
		{[]int{1, 2, 3, 4, 5}, 3, []int{1, 2, 3, 4, 5}, "Already partitioned"},
	}

	fmt.Println("=== Partition Array According to Given Pivot ===")

	for _, tc := range testCases {
		result := pivotArray(tc.nums, tc.pivot)
		status := "✓"
		if fmt.Sprint(result) != fmt.Sprint(tc.expected) {
			status = "✗"
		}

		fmt.Printf(
			"%s: nums=%v, pivot=%d -> %v %s (expected %v)\n",
			tc.name, tc.nums, tc.pivot, result, status, tc.expected,
		)
	}
}
