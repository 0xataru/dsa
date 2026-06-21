/// Maximum number of ice cream bars the boy can buy.
///
/// To buy as many bars as possible we greedily prefer the cheapest ones. The
/// problem mandates a counting sort, which fits perfectly because prices are
/// bounded by `1 <= costs[i] <= 100_000`: we tally how many bars exist at each
/// price into a fixed-size frequency table, then sweep prices from cheapest to
/// most expensive. At each price we buy as many bars as we can afford, capped
/// by how many bars are actually available at that price.
///
/// Once `coins` drops below the current price, every later price is strictly
/// larger, so nothing more can ever be bought and we stop early.
///
/// Time: O(n + M) where M = 100_000 is the max price.
/// Space: O(M) for the frequency table.
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
            // Cannot afford this price, nor any larger one that follows.
            break;
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

#[derive(Debug)]
struct TestCase {
    costs: Vec<i32>,
    coins: i32,
    expected: i32,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            costs: vec![1, 3, 2, 4, 1],
            coins: 7,
            expected: 4,
            name: "Example 1",
        },
        TestCase {
            costs: vec![10, 6, 8, 7, 7, 8],
            coins: 5,
            expected: 0,
            name: "Example 2",
        },
        TestCase {
            costs: vec![1, 6, 3, 1, 2, 5],
            coins: 20,
            expected: 6,
            name: "Example 3",
        },
        TestCase {
            costs: vec![1],
            coins: 1,
            expected: 1,
            name: "Single affordable bar",
        },
        TestCase {
            costs: vec![5, 5, 5, 5],
            coins: 12,
            expected: 2,
            name: "Duplicate prices, partial buy",
        },
        TestCase {
            costs: vec![100_000, 1, 1],
            coins: 2,
            expected: 2,
            name: "Skip the expensive one",
        },
    ]
}

fn main() {
    println!("=== Maximum Ice Cream Bars ===\n");

    for tc in test_cases() {
        let result = max_ice_cream(tc.costs.clone(), tc.coins);
        let status = if result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: costs={:?}, coins={} -> {} {} (expected {})",
            tc.name, tc.costs, tc.coins, result, status, tc.expected
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_match() {
        for tc in test_cases() {
            let result = max_ice_cream(tc.costs.clone(), tc.coins);
            assert_eq!(
                result, tc.expected,
                "{}: got {}, expected {}",
                tc.name, result, tc.expected
            );
        }
    }
}