package main

import (
	"fmt"
	"time"
)

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

// addTwoNumbers adds two numbers represented as linked lists
// Time Complexity: O(max(m, n)) - where m and n are lengths of the two lists
// Space Complexity: O(max(m, n)) - for the result list
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	dummyHead := &ListNode{Val: 0}
	current := dummyHead
	carry := 0

	for l1 != nil || l2 != nil || carry != 0 {
		x := 0
		if l1 != nil {
			x = l1.Val
			l1 = l1.Next
		}

		y := 0
		if l2 != nil {
			y = l2.Val
			l2 = l2.Next
		}

		sum := carry + x + y
		carry = sum / 10
		current.Next = &ListNode{Val: sum % 10}
		current = current.Next
	}

	return dummyHead.Next
}

// Helper function to create linked list from slice
func sliceToList(slice []int) *ListNode {
	if len(slice) == 0 {
		return nil
	}

	head := &ListNode{Val: slice[0]}
	current := head

	for i := 1; i < len(slice); i++ {
		current.Next = &ListNode{Val: slice[i]}
		current = current.Next
	}

	return head
}

// Helper function to convert linked list to slice for easy display
func listToSlice(head *ListNode) []int {
	var result []int
	current := head

	for current != nil {
		result = append(result, current.Val)
		current = current.Next
	}

	return result
}

// Helper function to calculate number from reverse digit slice
func sliceToNumber(slice []int) int64 {
	var result int64 = 0
	multiplier := int64(1)

	for _, digit := range slice {
		result += int64(digit) * multiplier
		multiplier *= 10
	}

	return result
}

func main() {
	// Test cases
	testCases := []struct {
		l1   []int
		l2   []int
		name string
	}{
		{[]int{2, 4, 3}, []int{5, 6, 4}, "Example 1"},
		{[]int{0}, []int{0}, "Example 2"},
		{[]int{9, 9, 9, 9, 9, 9, 9}, []int{9, 9, 9, 9}, "Example 3"},
		{[]int{1, 8}, []int{0}, "One list is shorter"},
		{[]int{5}, []int{5}, "Single digit with carry"},
		{[]int{1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1},
			[]int{5, 6, 4}, "Very long first list"},
	}

	for _, tc := range testCases {
		l1 := sliceToList(tc.l1)
		l2 := sliceToList(tc.l2)
		result := addTwoNumbers(l1, l2)
		resultSlice := listToSlice(result)

		fmt.Printf("%s: l1=%v, l2=%v -> result=%v\n",
			tc.name, tc.l1, tc.l2, resultSlice)

		// Calculate expected result for verification
		num1 := sliceToNumber(tc.l1)
		num2 := sliceToNumber(tc.l2)
		expectedSum := num1 + num2
		actualSum := sliceToNumber(resultSlice)

		fmt.Printf("  Verification: %d + %d = %d ✓\n\n",
			num1, num2, actualSum)

		if expectedSum != actualSum {
			fmt.Printf("  ERROR: Expected %d but got %d\n", expectedSum, actualSum)
		}
	}

	// Performance test with larger input
	fmt.Println("=== Performance Test ===")
	largeL1 := make([]int, 100)
	largeL2 := make([]int, 98)

	for i := 0; i < 100; i++ {
		largeL1[i] = i % 10
	}
	for i := 0; i < 98; i++ {
		largeL2[i] = (i + 1) % 10
	}

	start := time.Now()
	l1 := sliceToList(largeL1)
	l2 := sliceToList(largeL2)
	result := addTwoNumbers(l1, l2)
	resultSlice := listToSlice(result)
	duration := time.Since(start)

	fmt.Printf("Large input (100 + 98 digits): result length=%d\n", len(resultSlice))
	fmt.Printf("Execution time: %v\n", duration)

	// Micro-benchmark: measure only the algorithm execution
	fmt.Println("\n=== Micro-benchmark ===")

	// Pre-create test data to avoid measuring allocation in the loop
	type TestData struct {
		l1, l2 *ListNode
	}

	testData := make([]TestData, 1000)
	for i := 0; i < 1000; i++ {
		l1 := sliceToList([]int{2, 4, 3})
		l2 := sliceToList([]int{5, 6, 4})
		testData[i] = TestData{l1, l2}
	}

	iterations := 100000
	start = time.Now()
	for i := 0; i < iterations; i++ {
		data := testData[i%1000]
		// Clone the lists for each iteration
		l1 := cloneList(data.l1)
		l2 := cloneList(data.l2)
		_ = addTwoNumbers(l1, l2)
	}
	duration = time.Since(start)
	avgTime := duration / time.Duration(iterations)

	fmt.Printf("Average time per operation (100k iterations): %v\n", avgTime)
	fmt.Printf("Operations per second: %.0f\n", 1.0/avgTime.Seconds())
}

// Helper function to clone a linked list
func cloneList(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	newHead := &ListNode{Val: head.Val}
	current := newHead
	original := head.Next

	for original != nil {
		current.Next = &ListNode{Val: original.Val}
		current = current.Next
		original = original.Next
	}

	return newHead
}
