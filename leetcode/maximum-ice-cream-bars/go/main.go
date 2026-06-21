package main

import "fmt"

// maxIceCream returns the maximum number of ice cream bars buyable within the
// given budget. Prices are bounded by 1 <= costs[i] <= 100000, so we use a
// counting sort: tally how many bars exist at each price, then sweep prices
// from cheapest to most expensive, buying as many as the budget allows (capped
// by stock) at each step. Buying cheapest-first maximises the count.
//
// Once coins drops below the current price, every later price is strictly
// larger, so nothing more can be bought and we stop early.
//
// Time: O(n + M) where M = 100000 is the max price. Space: O(M).
func maxIceCream(costs []int, coins int) int {
	const maxCost = 100000

	freq := make([]int, maxCost+1)
	for _, cost := range costs {
		freq[cost]++
	}

	answer := 0
	for price := 1; price <= maxCost; price++ {
		if coins < price {
			// Cannot afford this price, nor any larger one that follows.
			break
		}
		if freq[price] == 0 {
			continue
		}

		canBuy := coins / price
		if freq[price] < canBuy {
			canBuy = freq[price]
		}
		answer += canBuy
		coins -= canBuy * price
	}

	return answer
}

func main() {
	testCases := []struct {
		costs    []int
		coins    int
		expected int
		name     string
	}{
		{[]int{1, 3, 2, 4, 1}, 7, 4, "Example 1"},
		{[]int{10, 6, 8, 7, 7, 8}, 5, 0, "Example 2"},
		{[]int{1, 6, 3, 1, 2, 5}, 20, 6, "Example 3"},
		{[]int{1}, 1, 1, "Single affordable bar"},
		{[]int{5, 5, 5, 5}, 12, 2, "Duplicate prices, partial buy"},
		{[]int{100000, 1, 1}, 2, 2, "Skip the expensive one"},
	}

	fmt.Println("=== Maximum Ice Cream Bars ===")

	for _, tc := range testCases {
		result := maxIceCream(tc.costs, tc.coins)
		status := "✓"
		if result != tc.expected {
			status = "✗"
		}
		fmt.Printf("%s: costs=%v, coins=%d -> %d %s (expected %d)\n",
			tc.name, tc.costs, tc.coins, result, status, tc.expected)
	}
}