package main

import (
	"fmt"
	"sort"
)

type ride struct {
	start    int
	duration int
	finish   int
}

// earliestFinishTimeBrute tries every land/water pair in both orders.
// Time: O(n * m), Space: O(1)
func earliestFinishTimeBrute(
	landStartTime []int,
	landDuration []int,
	waterStartTime []int,
	waterDuration []int,
) int {
	ans := int(^uint(0) >> 1)

	for i := 0; i < len(landStartTime); i++ {
		for j := 0; j < len(waterStartTime); j++ {
			landFinish := landStartTime[i] + landDuration[i]
			waterStart := max(landFinish, waterStartTime[j])
			ans = min(ans, waterStart+waterDuration[j])

			waterFinish := waterStartTime[j] + waterDuration[j]
			landStart := max(waterFinish, landStartTime[i])
			ans = min(ans, landStart+landDuration[i])
		}
	}

	return ans
}

func buildRides(starts, durations []int) []ride {
	rides := make([]ride, len(starts))
	for i := range starts {
		rides[i] = ride{
			start:    starts[i],
			duration: durations[i],
			finish:   starts[i] + durations[i],
		}
	}
	sort.Slice(rides, func(i, j int) bool {
		return rides[i].start < rides[j].start
	})
	return rides
}

func preprocess(rides []ride) (prefixMinDuration, suffixMinFinish []int) {
	n := len(rides)
	prefixMinDuration = make([]int, n)
	suffixMinFinish = make([]int, n)

	prefixMinDuration[0] = rides[0].duration
	for i := 1; i < n; i++ {
		prefixMinDuration[i] = min(prefixMinDuration[i-1], rides[i].duration)
	}

	suffixMinFinish[n-1] = rides[n-1].finish
	for i := n - 2; i >= 0; i-- {
		suffixMinFinish[i] = min(suffixMinFinish[i+1], rides[i].finish)
	}

	return prefixMinDuration, suffixMinFinish
}

// bestFollowUp picks the best second ride after finishing the first at firstFinish.
func bestFollowUp(
	firstFinish int,
	rides []ride,
	prefixMinDuration, suffixMinFinish []int,
) int {
	idx := sort.Search(len(rides), func(i int) bool {
		return rides[i].start > firstFinish
	})

	ans := int(^uint(0) >> 1)

	if idx > 0 {
		ans = min(ans, firstFinish+prefixMinDuration[idx-1])
	}
	if idx < len(rides) {
		ans = min(ans, suffixMinFinish[idx])
	}

	return ans
}

// earliestFinishTimeOptimized sorts rides and uses prefix/suffix minima with binary search.
// Time: O((n + m) log(n + m)), Space: O(n + m)
func earliestFinishTimeOptimized(
	landStartTime []int,
	landDuration []int,
	waterStartTime []int,
	waterDuration []int,
) int {
	landRides := buildRides(landStartTime, landDuration)
	waterRides := buildRides(waterStartTime, waterDuration)

	waterPrefix, waterSuffix := preprocess(waterRides)
	landPrefix, landSuffix := preprocess(landRides)

	ans := int(^uint(0) >> 1)

	for _, land := range landRides {
		ans = min(ans, bestFollowUp(land.finish, waterRides, waterPrefix, waterSuffix))
	}

	for _, water := range waterRides {
		ans = min(ans, bestFollowUp(water.finish, landRides, landPrefix, landSuffix))
	}

	return ans
}

func main() {
	testCases := []struct {
		landStartTime  []int
		landDuration   []int
		waterStartTime []int
		waterDuration  []int
		expected       int
		name           string
	}{
		{
			[]int{2, 8}, []int{4, 1}, []int{6}, []int{3}, 9,
			"Example 1",
		},
		{
			[]int{5}, []int{3}, []int{1}, []int{10}, 14,
			"Example 2",
		},
		{
			[]int{1}, []int{1}, []int{1}, []int{1}, 3,
			"Both rides open at the same time",
		},
		{
			[]int{10, 1}, []int{5, 2}, []int{3, 8}, []int{4, 1}, 7,
			"Multiple rides, best pair is land 1 then water 0",
		},
	}

	fmt.Println("=== Brute Force O(n * m) ===")
	runTests(testCases, earliestFinishTimeBrute)

	fmt.Println("\n=== Optimized O((n + m) log(n + m)) ===")
	runTests(testCases, earliestFinishTimeOptimized)
}

func runTests(
	testCases []struct {
		landStartTime  []int
		landDuration   []int
		waterStartTime []int
		waterDuration  []int
		expected       int
		name           string
	},
	solve func([]int, []int, []int, []int) int,
) {
	for _, tc := range testCases {
		result := solve(
			tc.landStartTime,
			tc.landDuration,
			tc.waterStartTime,
			tc.waterDuration,
		)
		status := "✓"
		if result != tc.expected {
			status = "✗"
		}

		fmt.Printf(
			"%s: land=(%v, %v), water=(%v, %v) -> expected=%d, got=%d %s\n",
			tc.name,
			tc.landStartTime,
			tc.landDuration,
			tc.waterStartTime,
			tc.waterDuration,
			tc.expected,
			result,
			status,
		)
	}
}
