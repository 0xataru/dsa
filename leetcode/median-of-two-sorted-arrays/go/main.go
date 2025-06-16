package main

import (
	"fmt"
	"math"
	"time"
)

// findMedianSortedArrays finds median using binary search
// Time Complexity: O(log(min(m,n))) - binary search on smaller array
// Space Complexity: O(1) - constant extra space
func findMedianSortedArrays(nums1, nums2 []int) float64 {
	// Ensure nums1 is the smaller array
	if len(nums1) > len(nums2) {
		nums1, nums2 = nums2, nums1
	}

	m, n := len(nums1), len(nums2)
	total := m + n
	half := (total + 1) / 2

	left, right := 0, m

	for left <= right {
		partition1 := (left + right) / 2
		partition2 := half - partition1

		// Get boundary values
		maxLeft1 := math.MinInt32
		if partition1 > 0 {
			maxLeft1 = nums1[partition1-1]
		}

		minRight1 := math.MaxInt32
		if partition1 < m {
			minRight1 = nums1[partition1]
		}

		maxLeft2 := math.MinInt32
		if partition2 > 0 {
			maxLeft2 = nums2[partition2-1]
		}

		minRight2 := math.MaxInt32
		if partition2 < n {
			minRight2 = nums2[partition2]
		}

		// Check if we found the correct partition
		if maxLeft1 <= minRight2 && maxLeft2 <= minRight1 {
			// Found the correct partition
			if total%2 == 1 {
				// Odd total length - return middle element
				return float64(max(maxLeft1, maxLeft2))
			} else {
				// Even total length - return average of two middle elements
				leftMax := max(maxLeft1, maxLeft2)
				rightMin := min(minRight1, minRight2)
				return float64(leftMax+rightMin) / 2.0
			}
		} else if maxLeft1 > minRight2 {
			// Too far on right side of nums1, move left
			right = partition1 - 1
		} else {
			// Too far on left side of nums1, move right
			left = partition1 + 1
		}
	}

	// Should never reach here
	return 0.0
}

// Helper functions for min/max
func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func main() {
	// Test cases
	testCases := []struct {
		nums1    []int
		nums2    []int
		expected float64
		name     string
	}{
		{[]int{1, 3}, []int{2}, 2.0, "Example 1"},
		{[]int{1, 2}, []int{3, 4}, 2.5, "Example 2"},
		{[]int{}, []int{1}, 1.0, "Empty first array"},
		{[]int{2}, []int{}, 2.0, "Empty second array"},
		{[]int{1, 3, 5, 7, 9}, []int{2, 4, 6, 8, 10}, 5.5, "Equal length arrays"},
		{[]int{1, 2}, []int{1, 2}, 1.5, "Duplicate values"},
		{[]int{-5, -3, -1}, []int{-2, 0, 4}, -1.5, "Negative numbers"},
	}

	fmt.Println("=== Median of Two Sorted Arrays - Binary Search Solution ===")

	for _, tc := range testCases {
		result := findMedianSortedArrays(tc.nums1, tc.nums2)
		passed := math.Abs(result-tc.expected) < 1e-9

		fmt.Printf("%s: nums1=%v, nums2=%v\n", tc.name, tc.nums1, tc.nums2)
		fmt.Printf("  Expected: %.5f, Got: %.5f %s\n",
			tc.expected, result, func() string {
				if passed {
					return "✓"
				}
				return "✗"
			}())

		if !passed {
			fmt.Println("  ERROR: Test case failed!")
		}
		fmt.Println()
	}

	// Performance test
	fmt.Println("=== Performance Test ===")

	// Create large test arrays
	var largeNums1, largeNums2 []int
	for i := 1; i <= 500; i += 2 {
		largeNums1 = append(largeNums1, i) // [1, 3, 5, ...]
	}
	for i := 2; i <= 500; i += 2 {
		largeNums2 = append(largeNums2, i) // [2, 4, 6, ...]
	}

	start := time.Now()
	result := findMedianSortedArrays(largeNums1, largeNums2)
	duration := time.Since(start)

	fmt.Printf("Large input (250 + 250 elements): median = %.5f\n", result)
	fmt.Printf("Execution time: %v\n", duration)
	fmt.Println("Time complexity: O(log(min(m,n))) = O(log(250)) ≈ O(8)")

	// Edge case performance
	verySmall := []int{1}
	var veryLarge []int
	for i := 2; i <= 1000; i++ {
		veryLarge = append(veryLarge, i)
	}

	start = time.Now()
	result = findMedianSortedArrays(verySmall, veryLarge)
	duration = time.Since(start)

	fmt.Printf("\nEdge case (1 + 999 elements): median = %.5f\n", result)
	fmt.Printf("Execution time: %v\n", duration)
	fmt.Println("Time complexity: O(log(min(1,999))) = O(log(1)) = O(1)")
}
