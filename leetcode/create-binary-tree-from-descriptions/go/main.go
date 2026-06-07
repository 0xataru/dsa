package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// createBinaryTree builds a tree from [parent, child, isLeft] descriptions.
// Time: O(n), Space: O(n)
func createBinaryTree(descriptions [][]int) *TreeNode {
	nodes := make(map[int]*TreeNode)
	children := make(map[int]bool)

	getNode := func(val int) *TreeNode {
		if node, ok := nodes[val]; ok {
			return node
		}
		node := &TreeNode{Val: val}
		nodes[val] = node
		return node
	}

	for _, desc := range descriptions {
		parent := getNode(desc[0])
		child := getNode(desc[1])

		if desc[2] == 1 {
			parent.Left = child
		} else {
			parent.Right = child
		}

		children[desc[1]] = true
	}

	for _, desc := range descriptions {
		parentVal := desc[0]
		if !children[parentVal] {
			return nodes[parentVal]
		}
	}

	return nil
}

func treeToSlice(root *TreeNode) []string {
	if root == nil {
		return nil
	}

	result := []string{}
	queue := []*TreeNode{root}

	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]

		if node == nil {
			result = append(result, "null")
			continue
		}

		result = append(result, fmt.Sprintf("%d", node.Val))
		queue = append(queue, node.Left, node.Right)
	}

	for len(result) > 0 && result[len(result)-1] == "null" {
		result = result[:len(result)-1]
	}

	return result
}

func main() {
	testCases := []struct {
		descriptions [][]int
		expected     []string
		name         string
	}{
		{
			[][]int{{20, 15, 1}, {20, 17, 0}, {50, 20, 1}, {50, 80, 0}, {80, 19, 1}},
			[]string{"50", "20", "80", "15", "17", "19"},
			"Example 1",
		},
		{
			[][]int{{1, 2, 1}, {2, 3, 0}, {3, 4, 1}},
			[]string{"1", "2", "null", "null", "3", "4"},
			"Example 2",
		},
		{
			[][]int{{10, 5, 1}, {10, 15, 0}},
			[]string{"10", "5", "15"},
			"Simple two children",
		},
	}

	fmt.Println("=== Create Binary Tree From Descriptions ===")

	for _, tc := range testCases {
		root := createBinaryTree(tc.descriptions)
		result := treeToSlice(root)
		status := "✓"
		if fmt.Sprint(result) != fmt.Sprint(tc.expected) {
			status = "✗"
		}

		fmt.Printf(
			"%s: descriptions=%v -> %v %s (expected %v)\n",
			tc.name, tc.descriptions, result, status, tc.expected,
		)
	}
}
