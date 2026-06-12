package main

import "fmt"

const mod = 1_000_000_007

// assignEdgeWeights counts odd-cost weight assignments on the root-to-deepest path.
// Time: O(n), Space: O(n)
func assignEdgeWeights(edges [][]int) int {
	n := len(edges) + 1
	graph := make([][]int, n+1)

	for _, edge := range edges {
		u, v := edge[0], edge[1]
		graph[u] = append(graph[u], v)
		graph[v] = append(graph[v], u)
	}

	maxDepth := 0
	stack := []struct {
		node, parent, depth int
	}{{1, 0, 0}}

	for len(stack) > 0 {
		cur := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		if cur.depth > maxDepth {
			maxDepth = cur.depth
		}

		for _, next := range graph[cur.node] {
			if next != cur.parent {
				stack = append(stack, struct {
					node, parent, depth int
				}{next, cur.node, cur.depth + 1})
			}
		}
	}

	exp := maxDepth - 1
	if exp < 0 {
		exp = 0
	}
	return int(modPow(2, exp, mod))
}

func modPow(base, exp, modulo int) int {
	result := 1
	base %= modulo

	for exp > 0 {
		if exp&1 == 1 {
			result = result * base % modulo
		}
		base = base * base % modulo
		exp >>= 1
	}

	return result
}

func main() {
	testCases := []struct {
		edges    [][]int
		expected int
		name     string
	}{
		{[][]int{{1, 2}}, 1, "Example 1"},
		{[][]int{{1, 2}, {1, 3}, {3, 4}, {3, 5}}, 2, "Example 2"},
		{[][]int{{1, 2}, {2, 3}, {3, 4}}, 4, "Linear chain depth 3"},
	}

	fmt.Println("=== Number of Ways to Assign Edge Weights I ===")

	for _, tc := range testCases {
		result := assignEdgeWeights(tc.edges)
		status := "✓"
		if result != tc.expected {
			status = "✗"
		}

		fmt.Printf(
			"%s: edges=%v -> %d %s (expected %d)\n",
			tc.name, tc.edges, result, status, tc.expected,
		)
	}
}
