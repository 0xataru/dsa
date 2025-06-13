package main

import "fmt"

// twoSum finds indices of two numbers that add up to target
// Time Complexity: O(n) - we traverse the array once
// Space Complexity: O(n) - we store at most n elements in the hash map
func twoSum(nums []int, target int) []int {
	// Hash map to store value -> index mapping
	numMap := make(map[int]int)

	for i, num := range nums {
		complement := target - num

		// Check if complement exists in map
		if index, exists := numMap[complement]; exists {
			return []int{index, i}
		}

		// Store current number and its index
		numMap[num] = i
	}

	// This should never be reached given the problem constraints
	return nil
}

func main() {
	// Test cases
	testCases := []struct {
		nums   []int
		target int
		name   string
	}{
		{[]int{2, 7, 11, 15}, 9, "Example 1"},
		{[]int{3, 2, 4}, 6, "Example 2"},
		{[]int{3, 3}, 6, "Example 3"},
		{[]int{-1, -2, -3, -4, -5}, -8, "Negative numbers"},
		{[]int{0, 4, 3, 0}, 0, "Zero target"},
	}

	for _, tc := range testCases {
		result := twoSum(tc.nums, tc.target)
		fmt.Printf("%s: nums=%v, target=%d -> result=%v\n",
			tc.name, tc.nums, tc.target, result)

		// Verify the result
		if len(result) == 2 {
			sum := tc.nums[result[0]] + tc.nums[result[1]]
			fmt.Printf("  Verification: nums[%d] + nums[%d] = %d + %d = %d ✓\n\n",
				result[0], result[1], tc.nums[result[0]], tc.nums[result[1]], sum)
		}
	}
}
