const MOD: i64 = 1_000_000_007;

/// For each query, count weight assignments on u-v path with odd total cost.
/// Time: O(n log n + q log n), Space: O(n log n)
fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len() + 1;
    let log = 18; // enough for n <= 1e5

    let mut graph = vec![Vec::<usize>::new(); n + 1];
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut depth = vec![0usize; n + 1];
    let mut up = vec![vec![0usize; n + 1]; log];

    let mut stack = vec![(1usize, 0usize)];
    while let Some((u, parent)) = stack.pop() {
        up[0][u] = parent;

        for &v in &graph[u] {
            if v == parent {
                continue;
            }
            depth[v] = depth[u] + 1;
            stack.push((v, u));
        }
    }

    for k in 1..log {
        for v in 1..=n {
            let mid = up[k - 1][v];
            up[k][v] = up[k - 1][mid];
        }
    }

    let mut pow2 = vec![1i64; n + 1];
    for i in 1..=n {
        pow2[i] = pow2[i - 1] * 2 % MOD;
    }

    queries
        .into_iter()
        .map(|q| {
            let u = q[0] as usize;
            let v = q[1] as usize;
            let ancestor = lca(u, v, &depth, &up);
            let len = depth[u] + depth[v] - 2 * depth[ancestor];

            if len == 0 {
                0
            } else {
                pow2[len - 1] as i32
            }
        })
        .collect()
}

fn lca(mut u: usize, mut v: usize, depth: &[usize], up: &[Vec<usize>]) -> usize {
    let log = up.len();

    if depth[u] < depth[v] {
        std::mem::swap(&mut u, &mut v);
    }

    let diff = depth[u] - depth[v];
    for k in 0..log {
        if (diff >> k) & 1 == 1 {
            u = up[k][u];
        }
    }

    if u == v {
        return u;
    }

    for k in (0..log).rev() {
        if up[k][u] != up[k][v] {
            u = up[k][u];
            v = up[k][v];
        }
    }

    up[0][u]
}

#[derive(Debug)]
struct TestCase {
    edges: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
    expected: Vec<i32>,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            edges: vec![vec![1, 2]],
            queries: vec![vec![1, 1], vec![1, 2]],
            expected: vec![0, 1],
            name: "Example 1",
        },
        TestCase {
            edges: vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]],
            queries: vec![vec![1, 4], vec![3, 4], vec![2, 5]],
            expected: vec![2, 1, 4],
            name: "Example 2",
        },
    ]
}

fn main() {
    println!("=== Number of Ways to Assign Edge Weights II ===\n");

    for tc in test_cases() {
        let result = assign_edge_weights(tc.edges.clone(), tc.queries.clone());
        let status = if result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: edges={:?}, queries={:?} -> {:?} {} (expected {:?})",
            tc.name, tc.edges, tc.queries, result, status, tc.expected
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
                assign_edge_weights(tc.edges, tc.queries),
                tc.expected,
                "{}",
                tc.name
            );
        }
    }
}
