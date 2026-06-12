const MOD: i64 = 1_000_000_007;

/// Number of ways to assign weights 1/2 on root-to-deepest path so total cost is odd.
/// Time: O(n), Space: O(n)
fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
    let n = edges.len() + 1;
    let mut graph = vec![Vec::<usize>::new(); n + 1];

    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut max_depth = 0usize;
    let mut stack = vec![(1usize, 0usize, 0usize)]; // (node, parent, depth)

    while let Some((u, parent, depth)) = stack.pop() {
        max_depth = max_depth.max(depth);

        for &v in &graph[u] {
            if v != parent {
                stack.push((v, u, depth + 1));
            }
        }
    }

    mod_pow(2, (max_depth.saturating_sub(1)) as i64, MOD) as i32
}

fn mod_pow(mut base: i64, mut exp: i64, modulo: i64) -> i64 {
    let mut result = 1i64;

    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulo;
        }
        base = base * base % modulo;
        exp >>= 1;
    }

    result
}

#[derive(Debug)]
struct TestCase {
    edges: Vec<Vec<i32>>,
    expected: i32,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            edges: vec![vec![1, 2]],
            expected: 1,
            name: "Example 1",
        },
        TestCase {
            edges: vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]],
            expected: 2,
            name: "Example 2",
        },
        TestCase {
            edges: vec![vec![1, 2], vec![2, 3], vec![3, 4]],
            expected: 4,
            name: "Linear chain depth 3",
        },
    ]
}

fn main() {
    println!("=== Number of Ways to Assign Edge Weights I ===\n");

    for tc in test_cases() {
        let result = assign_edge_weights(tc.edges.clone());
        let status = if result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: edges={:?} -> {} {} (expected {})",
            tc.name, tc.edges, result, status, tc.expected
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_match() {
        for tc in test_cases() {
            assert_eq!(
                assign_edge_weights(tc.edges),
                tc.expected,
                "{}",
                tc.name
            );
        }
    }

    #[test]
    fn mod_pow_basic() {
        assert_eq!(mod_pow(2, 3, MOD), 8);
        assert_eq!(mod_pow(2, 0, MOD), 1);
    }
}
