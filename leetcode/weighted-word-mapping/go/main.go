package main

import "fmt"

func mapWordWeightsLoop(words []string, weights []int) string {
	result := make([]byte, 0, len(words))

	for _, word := range words {
		sum := 0
		for i := 0; i < len(word); i++ {
			sum += weights[word[i]-'a']
		}
		result = append(result, byte('z'-(sum%26)))
	}

	return string(result)
}

func mapWordWeightsIter(words []string, weights []int) string {
	result := make([]byte, len(words))

	for i, word := range words {
		sum := 0
		for j := 0; j < len(word); j++ {
			sum += weights[word[j]-'a']
		}
		result[i] = byte('z' - (sum % 26))
	}

	return string(result)
}

func main() {
	testCases := []struct {
		words    []string
		weights  []int
		expected string
		name     string
	}{
		{
			[]string{"abcd", "def", "xyz"},
			[]int{5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2},
			"rij",
			"Example 1",
		},
		{
			[]string{"a", "b", "c"},
			func() []int {
				w := make([]int, 26)
				for i := range w {
					w[i] = 1
				}
				return w
			}(),
			"yyy",
			"Example 2",
		},
		{
			[]string{"abcd"},
			[]int{7, 5, 3, 4, 3, 5, 4, 9, 4, 2, 2, 7, 10, 2, 5, 10, 6, 1, 2, 2, 4, 1, 3, 4, 4, 5},
			"g",
			"Example 3",
		},
	}

	fmt.Println("=== Weighted Word Mapping ===")

	for _, tc := range testCases {
		loopResult := mapWordWeightsLoop(tc.words, tc.weights)
		iterResult := mapWordWeightsIter(tc.words, tc.weights)

		loopStatus := "✓"
		if loopResult != tc.expected {
			loopStatus = "✗"
		}
		iterStatus := "✓"
		if iterResult != tc.expected {
			iterStatus = "✗"
		}

		fmt.Printf(
			"%s: words=%v -> loop=%q %s, iter=%q %s (expected %q)\n",
			tc.name, tc.words, loopResult, loopStatus, iterResult, iterStatus, tc.expected,
		)
	}
}
