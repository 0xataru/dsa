/// Compute the smaller angle (in degrees) between the hour and minute hands.
///
/// The minute hand sweeps 360° in 60 minutes, so it sits at `minutes * 6°`.
/// The hour hand sweeps 360° in 12 hours, i.e. 30° per hour, and it also drifts
/// as the minutes pass: `0.5°` per minute. We take `hour % 12` so that 12
/// maps to 0°. The raw difference can exceed 180°, so we fold it back by
/// comparing against `360 - diff` and keeping the smaller value.
///
/// Time: O(1).
/// Space: O(1).
fn angle_clock(hour: i32, minutes: i32) -> f64 {
    let minute_angle = minutes as f64 * 6.0;
    let hour_angle = (hour % 12) as f64 * 30.0 + minutes as f64 * 0.5;

    let diff = (hour_angle - minute_angle).abs();

    diff.min(360.0 - diff)
}

#[derive(Debug)]
struct TestCase {
    hour: i32,
    minutes: i32,
    expected: f64,
    name: &'static str,
}

fn test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            hour: 12,
            minutes: 30,
            expected: 165.0,
            name: "Example 1",
        },
        TestCase {
            hour: 3,
            minutes: 30,
            expected: 75.0,
            name: "Example 2",
        },
        TestCase {
            hour: 3,
            minutes: 15,
            expected: 7.5,
            name: "Example 3",
        },
        TestCase {
            hour: 12,
            minutes: 0,
            expected: 0.0,
            name: "Hands overlap at noon",
        },
        TestCase {
            hour: 6,
            minutes: 0,
            expected: 180.0,
            name: "Straight line",
        },
        TestCase {
            hour: 1,
            minutes: 57,
            expected: 76.5,
            name: "Reflex folded to smaller angle",
        },
    ]
}

fn main() {
    println!("=== Angle Between Hands of a Clock ===\n");

    for tc in test_cases() {
        let result = angle_clock(tc.hour, tc.minutes);
        let status = if (result - tc.expected).abs() < 1e-5 {
            "✓"
        } else {
            "✗"
        };

        println!(
            "{}: hour={}, minutes={} -> {} {} (expected {})",
            tc.name, tc.hour, tc.minutes, result, status, tc.expected
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_match() {
        for tc in test_cases() {
            let result = angle_clock(tc.hour, tc.minutes);
            assert!(
                (result - tc.expected).abs() < 1e-5,
                "{}: got {}, expected {}",
                tc.name,
                result,
                tc.expected
            );
        }
    }
}