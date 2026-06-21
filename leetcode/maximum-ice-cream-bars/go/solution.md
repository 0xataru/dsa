# Maximum Ice Cream Bars - Go Solution

## Problem Summary
Given prices `costs` and a budget `coins`, buy as many ice cream bars as possible. The problem explicitly requires a **counting sort** based approach.

## Solution: Counting Sort + Greedy

To maximise the *count* of bars, always prefer the cheapest available bar. Because prices are tightly bounded (`1 <= costs[i] <= 100000`), we replace an `O(n log n)` sort with a linear counting sort:

1. **Tally** every price into a fixed frequency table `freq[price]`.
2. **Sweep** prices from `1` upward (cheapest first). At each price, buy as many bars as the budget allows, capped by stock at that price: `min(freq[price], coins / price)`.
3. **Early exit**: the moment `coins < price`, no larger price could ever be afforded, so we stop.

```go
func maxIceCream(costs []int, coins int) int {
    const maxCost = 100000

    freq := make([]int, maxCost+1)
    for _, cost := range costs {
        freq[cost]++
    }

    answer := 0
    for price := 1; price <= maxCost; price++ {
        if coins < price {
            break // cannot afford this price nor any larger one
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
```

### Why buying the cheapest first is optimal
Swapping any chosen expensive bar for an unchosen cheaper one never reduces the count and never increases the spend, so a cheapest-first selection is always at least as good as any other selection of the same size. Greedily taking the cheapest bars therefore maximises the total bought.

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n + M), M = 100000 |
| Space | O(M) |