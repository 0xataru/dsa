# Maximum Ice Cream Bars - Rust Solution

## Problem Summary
Given prices `costs` and a budget `coins`, buy as many ice cream bars as possible. The problem explicitly requires a **counting sort** based approach.

## Solution: Counting Sort + Greedy

To maximise the *count* of bars, always prefer the cheapest available bar. The standard greedy is "sort, then buy from the front." But because prices are tightly bounded (`1 <= costs[i] <= 100_000`), we can replace the `O(n log n)` comparison sort with a linear counting sort:

1. **Tally** every price into a fixed frequency table `freq[price]`.
2. **Sweep** prices from `1` upward (cheapest first). At each price, buy as many bars as the budget allows, capped by the number actually in stock at that price: `min(freq[price], coins / price)`.
3. **Early exit**: the moment `coins < price`, no larger price could ever be afforded, so we stop.

```rust
fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
    const MAX_COST: usize = 100_000;

    let mut freq = vec![0i32; MAX_COST + 1];
    for cost in costs {
        freq[cost as usize] += 1;
    }

    let mut coins = coins;
    let mut answer = 0;

    for price in 1..=MAX_COST {
        if coins < price as i32 {
            break; // cannot afford this price nor any larger one
        }
        if freq[price] == 0 {
            continue;
        }

        let can_buy = freq[price].min(coins / price as i32);
        answer += can_buy;
        coins -= can_buy * price as i32;
    }

    answer
}
```

### Why buying the cheapest first is optimal
Swapping any chosen expensive bar for an unchosen cheaper one never reduces the count and never increases the spend, so a cheapest-first selection is always at least as good as any other selection of the same size. Hence greedily taking the cheapest bars maximises the total number bought.

### Note on the original draft
The original solution buys in bulk per price with `min(freq[price], coins / price)` — correct and elegant. The only refinement here is that the budget-exhaustion guard should `break` (not `continue`): since prices are visited in increasing order, once `coins < price` every subsequent price is also unaffordable, so there is nothing left to do.

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(n + M), M = 100_000 |
| Space | O(M) |