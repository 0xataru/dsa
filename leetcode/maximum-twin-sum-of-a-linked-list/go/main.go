package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func vecToList(values []int) *ListNode {
	dummy := &ListNode{}
	cur := dummy
	for _, v := range values {
		cur.Next = &ListNode{Val: v}
		cur = cur.Next
	}
	return dummy.Next
}

// pairSumSlice copies values, pairs from both ends.
// Time: O(n), Space: O(n)
func pairSumSlice(head *ListNode) int {
	nums := make([]int, 0)
	for cur := head; cur != nil; cur = cur.Next {
		nums = append(nums, cur.Val)
	}

	ans := 0
	n := len(nums)
	for i := 0; i < n/2; i++ {
		sum := nums[i] + nums[n-1-i]
		if sum > ans {
			ans = sum
		}
	}
	return ans
}

func reverseList(head *ListNode) *ListNode {
	var prev *ListNode
	for head != nil {
		next := head.Next
		head.Next = prev
		prev = head
		head = next
	}
	return prev
}

// pairSumInplace splits at middle, reverses second half, compares pairs.
// Time: O(n), Space: O(1) extra
func pairSumInplace(head *ListNode) int {
	length := 0
	for cur := head; cur != nil; cur = cur.Next {
		length++
	}

	// Advance to the last node of the first half, then cut the list.
	mid := head
	for i := 0; i < length/2-1; i++ {
		mid = mid.Next
	}

	second := mid.Next
	mid.Next = nil
	second = reverseList(second)

	first := head
	ans := 0

	for first != nil && second != nil {
		sum := first.Val + second.Val
		if sum > ans {
			ans = sum
		}
		first = first.Next
		second = second.Next
	}

	return ans
}

func main() {
	testCases := []struct {
		values   []int
		expected int
		name     string
	}{
		{[]int{5, 4, 2, 1}, 6, "Example 1"},
		{[]int{4, 2, 2, 3}, 7, "Example 2"},
		{[]int{1, 100000}, 100001, "Example 3"},
	}

	fmt.Println("=== Maximum Twin Sum of a Linked List ===")

	for _, tc := range testCases {
		sliceResult := pairSumSlice(vecToList(tc.values))
		inplaceResult := pairSumInplace(vecToList(tc.values))

		sliceStatus := "✓"
		if sliceResult != tc.expected {
			sliceStatus = "✗"
		}
		inplaceStatus := "✓"
		if inplaceResult != tc.expected {
			inplaceStatus = "✗"
		}

		fmt.Printf(
			"%s: %v -> slice=%d %s, inplace=%d %s (expected %d)\n",
			tc.name, tc.values, sliceResult, sliceStatus, inplaceResult, inplaceStatus, tc.expected,
		)
	}
}
