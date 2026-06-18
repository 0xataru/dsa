# Angle Between Hands of a Clock - Go Solution

## Problem Summary
Given `hour` and `minutes`, return the smaller angle (in degrees) between the hour and minute hands of an analog clock.

## Solution: Direct Geometry

Place each hand at its angular position measured clockwise from 12 o'clock:

- **Minute hand**: a full revolution (360°) takes 60 minutes, so it sits at `minutes * 6°`.
- **Hour hand**: 360° over 12 hours is `30°` per hour, *plus* a continuous drift of `0.5°` per minute as the hour advances. Using `hour % 12` makes 12 map to 0°: `(hour % 12) * 30 + minutes * 0.5`.

The absolute difference of the two positions can be anywhere in `[0°, 360°)`. Since we want the *smaller* angle, fold any reflex angle by taking `min(diff, 360 - diff)`.

```go
func angleClock(hour int, minutes int) float64 {
    minuteAngle := float64(minutes) * 6.0
    hourAngle := float64(hour%12)*30.0 + float64(minutes)*0.5

    diff := math.Abs(hourAngle - minuteAngle)

    return math.Min(diff, 360.0-diff)
}
```

### Why the hour drift matters
At 12:30 the hour hand is not on 12 — it is halfway to 1, i.e. `30 * 0.5 = 15°` past 12. The minute hand is at `30 * 6 = 180°`. The difference is `165°`, which matches Example 1. Forgetting the `minutes * 0.5` term would wrongly give `180°`.

### Complexity

| Metric | Value |
|--------|-------|
| Time | O(1) |
| Space | O(1) |