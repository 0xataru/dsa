package main

import "fmt"

const mod = 1_000_000_007

func assignEdgeWeights(edges [][]int, queries [][]int) []int {
	n := len(edges) + 1
	const log = 18

	graph := make([][]int, n+1)
	for _, edge := range edges {
		u, v := edge[0], edge[1]
		graph[u] = append(graph[u], v)
		graph[v] = append(graph[v], u)
	}

	depth := make([]int, n+1)
	up := make([][]int, log)
	for i := range up {
		up[i] = make([]int, n+1)
	}

	stack := []struct{ node, parent int }{{1, 0}}
	for len(stack) > 0 {
		cur := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		up[0][cur.node] = cur.parent

		for _, next := range graph[cur.node] {
			if next == cur.parent {
				continue
			}
			depth[next] = depth[cur.node] + 1
			stack = append(stack, struct{ node, parent int }{next, cur.node})
		}
	}

	for k := 1; k < log; k++ {
		for v := 1; v <= n; v++ {
			mid := up[k-1][v]
			up[k][v] = up[k-1][mid]
		}
	}

	pow2 := make([]int, n+1)
	pow2[0] = 1
	for i := 1; i <= n; i++ {
		pow2[i] = pow2[i-1] * 2 % mod
	}

	answers := make([]int, len(queries))
	for i, q := range queries {
		u, v := q[0], q[1]
		ancestor := lca(u, v, depth, up)
		length := depth[u] + depth[v] - 2*depth[ancestor]

		if length == 0 {
			answers[i] = 0
		} else {
			answers[i] = pow2[length-1]
		}
	}

	return answers
}

func lca(u, v int, depth []int, up [][]int) int {
	log := len(up)

	if depth[u] < depth[v] {
		u, v = v, u
	}

	diff := depth[u] - depth[v]
	for k := 0; k < log; k++ {
		if (diff>>k)&1 == 1 {
			u = up[k][u]
		}
	}

	if u == v {
		return u
	}

	for k := log - 1; k >= 0; k-- {
		if up[k][u] != up[k][v] {
			u = up[k][u]
			v = up[k][v]
		}
	}

	return up[0][u]
}

func main() {
	testCases := []struct {
		edges    [][]int
		queries  [][]int
		expected []int
		name     string
	}{
		{
			[][]int{{1, 2}},
			[][]int{{1, 1}, {1, 2}},
			[]int{0, 1},
			"Example 1",
		},
		{
			[][]int{{1, 2}, {1, 3}, {3, 4}, {3, 5}},
			[][]int{{1, 4}, {3, 4}, {2, 5}},
			[]int{2, 1, 4},
			"Example 2",
		},
	}

	fmt.Println("=== Number of Ways to Assign Edge Weights II ===")

	for _, tc := range testCases {
		result := assignEdgeWeights(tc.edges, tc.queries)
		status := "✓"
		if fmt.Sprint(result) != fmt.Sprint(tc.expected) {
			status = "✗"
		}

		fmt.Printf(
			"%s: edges=%v, queries=%v -> %v %s (expected %v)\n",
			tc.name, tc.edges, tc.queries, result, status, tc.expected,
		)
	}
}
