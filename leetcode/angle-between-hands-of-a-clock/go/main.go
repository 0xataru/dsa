package main

import (
	"fmt"
	"math"
)

// angleClock computes the smaller angle (in degrees) between the hour and
// minute hands. The minute hand sits at minutes*6 degrees (360 degrees per 60
// minutes). The hour hand sits at (hour%12)*30 degrees plus a 0.5 degree per
// minute drift as the hour advances. The raw difference can exceed 180
// degrees, so we fold any reflex angle with min(diff, 360-diff).
func angleClock(hour int, minutes int) float64 {
	minuteAngle := float64(minutes) * 6.0
	hourAngle := float64(hour%12)*30.0 + float64(minutes)*0.5

	diff := math.Abs(hourAngle - minuteAngle)

	return math.Min(diff, 360.0-diff)
}

func main() {
	testCases := []struct {
		hour     int
		minutes  int
		expected float64
		name     string
	}{
		{12, 30, 165.0, "Example 1"},
		{3, 30, 75.0, "Example 2"},
		{3, 15, 7.5, "Example 3"},
		{12, 0, 0.0, "Hands overlap at noon"},
		{6, 0, 180.0, "Straight line"},
		{1, 57, 76.5, "Reflex folded to smaller angle"},
	}

	fmt.Println("=== Angle Between Hands of a Clock ===")

	for _, tc := range testCases {
		result := angleClock(tc.hour, tc.minutes)
		status := "✓"
		if math.Abs(result-tc.expected) >= 1e-5 {
			status = "✗"
		}
		fmt.Printf("%s: hour=%d, minutes=%d -> %g %s (expected %g)\n",
			tc.name, tc.hour, tc.minutes, result, status, tc.expected)
	}
}