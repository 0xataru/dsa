package main

import "fmt"

// leftRightDifference returns |leftSum[i] - rightSum[i]| for each index.
// Time: O(n), Space: O(1) extra (excluding output)
func leftRightDifference(nums []int) []int {
	totalSum := 0
	for _, num := range nums {
		totalSum += num
	}

	leftSum := 0
	answer := make([]int, 0, len(nums))

	for _, num := range nums {
		rightSum := totalSum - leftSum - num
		diff := leftSum - rightSum
		if diff < 0 {
			diff = -diff
		}
		answer = append(answer, diff)
		leftSum += num
	}

	return answer
}

func main() {
	testCases := []struct {
		nums     []int
		expected []int
		name     string
	}{
		{[]int{10, 4, 8, 3}, []int{15, 1, 11, 22}, "Example 1"},
		{[]int{1}, []int{0}, "Example 2"},
		{[]int{1, 2, 3, 4, 5}, []int{14, 11, 6, 1, 10}, "Increasing sequence"},
		{[]int{5, 5, 5}, []int{10, 0, 10}, "Equal elements"},
	}

	fmt.Println("=== Left and Right Sum Differences ===")

	for _, tc := range testCases {
		result := leftRightDifference(tc.nums)
		status := "✓"
		if fmt.Sprint(result) != fmt.Sprint(tc.expected) {
			status = "✗"
		}

		fmt.Printf(
			"%s: nums=%v -> %v %s (expected %v)\n",
			tc.name, tc.nums, result, status, tc.expected,
		)
	}
}
