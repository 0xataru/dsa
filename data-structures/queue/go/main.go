package main

import (
	"fmt"
	"time"
)

func main() {
	fmt.Println("=== Test simple queue ===")
	testSimpleQueue()

	fmt.Println("\n" + "==========================")
	TestOptimalQueue()

	fmt.Println("\n" + "==========================")
	PerformanceComparison()
}

func PerformanceComparison() {
	fmt.Println("=== Performance comparison ===")

	sizes := []int{100, 1000, 10000}

	for _, size := range sizes {
		fmt.Printf("\n--- Test with %d elements ---\n", size)

		start := time.Now()
		simpleQ := newQueue[int]()
		for i := 0; i < size; i++ {
			simpleQ.Enqueue(i)
		}
		for i := 0; i < size; i++ {
			simpleQ.Dequeue()
		}
		simpleTime := time.Since(start)

		// Test optimal queue
		start = time.Now()
		optimalQ := NewOptimalQueue[int]()
		for i := 0; i < size; i++ {
			optimalQ.Enqueue(i)
		}
		for i := 0; i < size; i++ {
			optimalQ.Dequeue()
		}
		optimalTime := time.Since(start)

		fmt.Printf("Simple queue: %v\n", simpleTime)
		fmt.Printf("Optimal queue: %v\n", optimalTime)
		fmt.Printf("Speedup: %.2fx\n", float64(simpleTime)/float64(optimalTime))
	}
}
