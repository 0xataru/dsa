#[derive(Clone, Copy)]
struct Ride {
    start: i32,
    duration: i32,
    finish: i32,
}

/// Brute force: try every land/water pair in both orders.
/// Time: O(n * m), Space: O(1)
fn earliest_finish_time(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
) -> i32 {
    let mut ans = i32::MAX;

    for i in 0..land_start_time.len() {
        for j in 0..water_start_time.len() {
            let land_finish = land_start_time[i] + land_duration[i];
            let water_start = land_finish.max(water_start_time[j]);
            ans = ans.min(water_start + water_duration[j]);

            let water_finish = water_start_time[j] + water_duration[j];
            let land_start = water_finish.max(land_start_time[i]);
            ans = ans.min(land_start + land_duration[i]);
        }
    }

    ans
}

fn build_rides(starts: &[i32], durations: &[i32]) -> Vec<Ride> {
    let mut rides: Vec<Ride> = starts
        .iter()
        .zip(durations.iter())
        .map(|(&start, &duration)| Ride {
            start,
            duration,
            finish: start + duration,
        })
        .collect();
    rides.sort_by_key(|ride| ride.start);
    rides
}

fn preprocess(rides: &[Ride]) -> (Vec<i32>, Vec<i32>) {
    let n = rides.len();
    let mut prefix_min_duration = vec![i32::MAX; n];
    let mut suffix_min_finish = vec![i32::MAX; n];

    prefix_min_duration[0] = rides[0].duration;
    for i in 1..n {
        prefix_min_duration[i] = prefix_min_duration[i - 1].min(rides[i].duration);
    }

    suffix_min_finish[n - 1] = rides[n - 1].finish;
    for i in (0..n - 1).rev() {
        suffix_min_finish[i] = suffix_min_finish[i + 1].min(rides[i].finish);
    }

    (prefix_min_duration, suffix_min_finish)
}

/// After finishing the first ride at `first_finish`, pick the best second ride.
///
/// For rides with `start <= first_finish`: finish at `first_finish + duration`
/// For rides with `start > first_finish`: finish at `start + duration`
fn best_follow_up(
    first_finish: i32,
    rides: &[Ride],
    prefix_min_duration: &[i32],
    suffix_min_finish: &[i32],
) -> i32 {
    let idx = rides.partition_point(|ride| ride.start <= first_finish);

    let mut ans = i32::MAX;

    if idx > 0 {
        ans = ans.min(first_finish + prefix_min_duration[idx - 1]);
    }
    if idx < rides.len() {
        ans = ans.min(suffix_min_finish[idx]);
    }

    ans
}

/// Optimized: sort rides and use prefix/suffix minima with binary search.
/// Time: O((n + m) log(n + m)), Space: O(n + m)
fn earliest_finish_time_optimized(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
) -> i32 {
    let land_rides = build_rides(&land_start_time, &land_duration);
    let water_rides = build_rides(&water_start_time, &water_duration);

    let (water_prefix, water_suffix) = preprocess(&water_rides);
    let (land_prefix, land_suffix) = preprocess(&land_rides);

    let mut ans = i32::MAX;

    for land in &land_rides {
        ans = ans.min(best_follow_up(
            land.finish,
            &water_rides,
            &water_prefix,
            &water_suffix,
        ));
    }

    for water in &water_rides {
        ans = ans.min(best_follow_up(
            water.finish,
            &land_rides,
            &land_prefix,
            &land_suffix,
        ));
    }

    ans
}

#[derive(Debug)]
struct TestCase {
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
    expected: i32,
    name: &'static str,
}

fn main() {
    let test_cases = vec![
        TestCase {
            land_start_time: vec![2, 8],
            land_duration: vec![4, 1],
            water_start_time: vec![6],
            water_duration: vec![3],
            expected: 9,
            name: "Example 1",
        },
        TestCase {
            land_start_time: vec![5],
            land_duration: vec![3],
            water_start_time: vec![1],
            water_duration: vec![10],
            expected: 14,
            name: "Example 2",
        },
        TestCase {
            land_start_time: vec![1],
            land_duration: vec![1],
            water_start_time: vec![1],
            water_duration: vec![1],
            expected: 3,
            name: "Both rides open at the same time",
        },
        TestCase {
            land_start_time: vec![10, 1],
            land_duration: vec![5, 2],
            water_start_time: vec![3, 8],
            water_duration: vec![4, 1],
            expected: 7,
            name: "Multiple rides, best pair is land 1 then water 0",
        },
    ];

    println!("=== Brute Force O(n * m) ===\n");
    run_tests(&test_cases, earliest_finish_time);

    println!("\n=== Optimized O((n + m) log(n + m)) ===\n");
    run_tests(&test_cases, earliest_finish_time_optimized);
}

fn run_tests(test_cases: &[TestCase], solve: fn(Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) -> i32) {
    for tc in test_cases {
        let result = solve(
            tc.land_start_time.clone(),
            tc.land_duration.clone(),
            tc.water_start_time.clone(),
            tc.water_duration.clone(),
        );
        let status = if result == tc.expected { "✓" } else { "✗" };

        println!(
            "{}: land=({:?}, {:?}), water=({:?}, {:?}) -> expected={}, got={} {}",
            tc.name,
            tc.land_start_time,
            tc.land_duration,
            tc.water_start_time,
            tc.water_duration,
            tc.expected,
            result,
            status
        );
    }
}
