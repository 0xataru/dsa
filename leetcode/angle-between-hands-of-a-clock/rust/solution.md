# Angle Between Hands of a Clock - Rust Solution

## Problem Summary
Given `hour` and `minutes`, return the smaller angle (in degrees) between the hour and minute hands of an analog clock.

## Solution: Direct Geometry

Place each hand at its angular position measured clockwise from 12 o'clock:

- **Minute hand**: a full revolution (360°) takes 60 minutes, so it sits at `minutes * 6°`.
- **Hour hand**: 360° over 12 hours is `30°` per hour, *plus* a continuous drift of `0.5°` per minute as the hour advances. Using `hour % 12` makes 12 map to 0°: `(hour % 12) * 30 + minutes * 0.5`.

The absolute difference of the two positions can be anywhere in `[0°, 360°)`. Since we want the *smaller* angle, fold any reflex angle by taking `min(diff, 360 - diff)`.

```rust
fn angle_clock(hour: i32, minutes: i32) -> f64 {
    let minute_angle = minutes as f64 * 6.0;
    let hour_angle = (hour % 12) as f64 * 30.0 + minutes as f64 * 0.5;

    let diff = (hour_angle - minute_angle).abs();

    diff.min(360.0 - diff)
}
```

### Why the hour drift matters
At 12:30 the hour hand is not on 12 — it is halfway to 1, i.e. `30 * 0.5 = 15°` past 12. The minute hand is at `30 * 6 = 180°`. The difference is `165°`, which matches Example 1. Forgetting the `minutes * 0.5` term would wrongly give `180°`.

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(1) |
| Space | O(1) |