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

func listToVec(head *ListNode) []int {
	result := make([]int, 0)
	for cur := head; cur != nil; cur = cur.Next {
		result = append(result, cur.Val)
	}
	return result
}

// deleteMiddleCount removes node at index n/2 after counting length.
func deleteMiddleCount(head *ListNode) *ListNode {
	length := 0
	for cur := head; cur != nil; cur = cur.Next {
		length++
	}

	if length == 1 {
		return nil
	}

	mid := length / 2
	cur := head
	for i := 0; i < mid-1; i++ {
		cur = cur.Next
	}

	cur.Next = cur.Next.Next
	return head
}

// deleteMiddleTwoPointers uses slow/fast to find node before middle.
func deleteMiddleTwoPointers(head *ListNode) *ListNode {
	if head.Next == nil {
		return nil
	}

	slow, fast := head, head.Next
	for fast != nil && fast.Next != nil && fast.Next.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
	}

	slow.Next = slow.Next.Next
	return head
}

func main() {
	testCases := []struct {
		values   []int
		expected []int
		name     string
	}{
		{[]int{1, 3, 4, 7, 1, 2, 6}, []int{1, 3, 4, 1, 2, 6}, "Example 1"},
		{[]int{1, 2, 3, 4}, []int{1, 2, 4}, "Example 2"},
		{[]int{2, 1}, []int{2}, "Example 3"},
		{[]int{1}, []int{}, "Single node"},
	}

	fmt.Println("=== Delete the Middle Node of a Linked List ===")

	for _, tc := range testCases {
		countResult := listToVec(deleteMiddleCount(vecToList(tc.values)))
		twoPtrResult := listToVec(deleteMiddleTwoPointers(vecToList(tc.values)))

		countStatus := "✓"
		if fmt.Sprint(countResult) != fmt.Sprint(tc.expected) {
			countStatus = "✗"
		}
		twoPtrStatus := "✓"
		if fmt.Sprint(twoPtrResult) != fmt.Sprint(tc.expected) {
			twoPtrStatus = "✗"
		}

		fmt.Printf(
			"%s: %v -> count=%v %s, two_ptr=%v %s (expected %v)\n",
			tc.name, tc.values, countResult, countStatus, twoPtrResult, twoPtrStatus, tc.expected,
		)
	}
}
