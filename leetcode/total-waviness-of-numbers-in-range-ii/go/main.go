package main

import (
	"fmt"
	"strconv"
)

const none = 10

// totalWavinessBrute is only for small ranges (verification).
func totalWavinessBrute(num1, num2 int64) int64 {
	var waviness int64
	for num := num1; num <= num2; num++ {
		s := strconv.FormatInt(num, 10)
		if len(s) < 3 {
			continue
		}
		for i := 1; i < len(s)-1; i++ {
			prev := int(s[i-1] - '0')
			curr := int(s[i] - '0')
			next := int(s[i+1] - '0')
			if (curr > prev && curr > next) || (curr < prev && curr < next) {
				waviness++
			}
		}
	}
	return waviness
}

type dpKey struct {
	pos, prev, prevPrev int
	tight               bool
}

type allLenKey struct {
	length, pos, prev, prevPrev int
}

func dpBounded(digits []byte, pos int, tight bool, prev, prevPrev int, memo map[dpKey][2]int64) (int64, int64) {
	if pos == len(digits) {
		return 1, 0
	}

	key := dpKey{pos, prev, prevPrev, tight}
	if v, ok := memo[key]; ok {
		return v[0], v[1]
	}

	lo := 0
	if pos == 0 {
		lo = 1
	}
	hi := 9
	if tight {
		hi = int(digits[pos] - '0')
	}

	var totalCount, totalWaviness int64
	for d := lo; d <= hi; d++ {
		var add int64
		if pos >= 2 {
			if prev > prevPrev && prev > d {
				add = 1
			} else if prev < prevPrev && prev < d {
				add = 1
			}
		}

		npp := none
		if pos >= 1 {
			npp = prev
		}

		subCount, subWaviness := dpBounded(digits, pos+1, tight && d == hi, d, npp, memo)
		totalCount += subCount
		totalWaviness += add*subCount + subWaviness
	}

	memo[key] = [2]int64{totalCount, totalWaviness}
	return totalCount, totalWaviness
}

func dpAllLen(length, pos, prev, prevPrev int, memo map[allLenKey][2]int64) (int64, int64) {
	if pos == length {
		return 1, 0
	}

	key := allLenKey{length, pos, prev, prevPrev}
	if v, ok := memo[key]; ok {
		return v[0], v[1]
	}

	lo := 0
	if pos == 0 {
		lo = 1
	}

	var totalCount, totalWaviness int64
	for d := lo; d <= 9; d++ {
		var add int64
		if pos >= 2 {
			if prev > prevPrev && prev > d {
				add = 1
			} else if prev < prevPrev && prev < d {
				add = 1
			}
		}

		npp := none
		if pos >= 1 {
			npp = prev
		}

		subCount, subWaviness := dpAllLen(length, pos+1, d, npp, memo)
		totalCount += subCount
		totalWaviness += add*subCount + subWaviness
	}

	memo[key] = [2]int64{totalCount, totalWaviness}
	return totalCount, totalWaviness
}

func wavinessUpTo(n int64) int64 {
	if n <= 0 {
		return 0
	}
	if n < 100 {
		return totalWavinessBrute(0, n)
	}

	digits := []byte(strconv.FormatInt(n, 10))
	length := len(digits)

	var ans int64
	memoAll := make(map[allLenKey][2]int64)
	for l := 3; l < length; l++ {
		_, w := dpAllLen(l, 0, none, none, memoAll)
		ans += w
	}

	memoBounded := make(map[dpKey][2]int64)
	_, w := dpBounded(digits, 0, true, none, none, memoBounded)
	ans += w
	return ans
}

func totalWaviness(num1, num2 int64) int64 {
	return wavinessUpTo(num2) - wavinessUpTo(num1-1)
}

func main() {
	examples := []struct {
		num1, num2 int64
		expected   int64
		name       string
	}{
		{120, 130, 3, "Example 1"},
		{198, 202, 3, "Example 2"},
		{4848, 4848, 2, "Example 3"},
		{1, 99, 0, "All numbers have fewer than 3 digits"},
		{100, 100, 0, "Three digits, no peak or valley"},
	}

	fmt.Println("=== Digit DP (Part II) ===")
	for _, tc := range examples {
		result := totalWaviness(tc.num1, tc.num2)
		status := "✓"
		if result != tc.expected {
			status = "✗"
		}
		fmt.Printf(
			"%s: num1=%d, num2=%d -> expected=%d, got=%d %s\n",
			tc.name, tc.num1, tc.num2, tc.expected, result, status,
		)
	}

	fmt.Printf(
		"\nTLE case: num1=6613514, num2=9155102 -> %d\n",
		totalWaviness(6613514, 9155102),
	)

	fmt.Println("\n=== Brute vs DP cross-check (small ranges) ===")
	for _, c := range [][2]int64{{120, 130}, {198, 202}, {1010, 1011}, {1000, 1500}} {
		brute := totalWavinessBrute(c[0], c[1])
		fast := totalWaviness(c[0], c[1])
		status := "✓"
		if brute != fast {
			status = "✗"
		}
		fmt.Printf("[%d, %d]: brute=%d, dp=%d %s\n", c[0], c[1], brute, fast, status)
	}
}
