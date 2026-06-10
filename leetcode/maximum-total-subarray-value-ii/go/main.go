package main

import (
	"container/heap"
	"fmt"
)

type sparseTable struct {
	stMax [][]int
	stMin [][]int
	lg    []int
}

func buildSparseTable(nums []int) sparseTable {
	n := len(nums)
	lg := make([]int, n+1)
	for i := 2; i <= n; i++ {
		lg[i] = lg[i/2] + 1
	}

	maxLog := lg[n] + 1
	stMax := make([][]int, n)
	stMin := make([][]int, n)
	for i := range stMax {
		stMax[i] = make([]int, maxLog)
		stMin[i] = make([]int, maxLog)
		stMax[i][0] = nums[i]
		stMin[i][0] = nums[i]
	}

	for j := 1; j < maxLog; j++ {
		length := 1 << j
		half := length >> 1
		for i := 0; i+length <= n; i++ {
			stMax[i][j] = max(stMax[i][j-1], stMax[i+half][j-1])
			stMin[i][j] = min(stMin[i][j-1], stMin[i+half][j-1])
		}
	}

	return sparseTable{stMax: stMax, stMin: stMin, lg: lg}
}

func (st sparseTable) query(l, r int) int64 {
	k := st.lg[r-l+1]
	span := 1 << k

	mx := max(st.stMax[l][k], st.stMax[r+1-span][k])
	mn := min(st.stMin[l][k], st.stMin[r+1-span][k])
	return int64(mx - mn)
}

type subarray struct {
	val int64
	l   int
	r   int
}

type maxHeap []subarray

func (h maxHeap) Len() int           { return len(h) }
func (h maxHeap) Less(i, j int) bool { return h[i].val > h[j].val }
func (h maxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *maxHeap) Push(x any) {
	*h = append(*h, x.(subarray))
}

func (h *maxHeap) Pop() any {
	old := *h
	item := old[len(old)-1]
	*h = old[:len(old)-1]
	return item
}

// maxTotalValue returns the maximum sum of k distinct subarray (max - min) values.
func maxTotalValue(nums []int, k int) int64 {
	n := len(nums)
	if n == 0 || k == 0 {
		return 0
	}

	st := buildSparseTable(nums)
	h := &maxHeap{}

	for l := 0; l < n; l++ {
		heap.Push(h, subarray{val: st.query(l, n-1), l: l, r: n - 1})
	}

	var ans int64
	for i := 0; i < k && h.Len() > 0; i++ {
		cur := heap.Pop(h).(subarray)
		ans += cur.val

		if cur.r > cur.l {
			heap.Push(h, subarray{val: st.query(cur.l, cur.r-1), l: cur.l, r: cur.r - 1})
		}
	}

	return ans
}

func main() {
	testCases := []struct {
		nums     []int
		k        int
		expected int64
		name     string
	}{
		{[]int{1, 3, 2}, 2, 4, "Example 1"},
		{[]int{4, 2, 5, 1}, 3, 12, "Example 2"},
		{[]int{7}, 1, 0, "Single element"},
		{[]int{0, 100}, 1, 100, "Two elements, best subarray"},
	}

	fmt.Println("=== Maximum Total Subarray Value II ===")

	for _, tc := range testCases {
		result := maxTotalValue(tc.nums, tc.k)
		status := "✓"
		if result != tc.expected {
			status = "✗"
		}

		fmt.Printf(
			"%s: nums=%v, k=%d -> %d %s (expected %d)\n",
			tc.name, tc.nums, tc.k, result, status, tc.expected,
		)
	}
}
