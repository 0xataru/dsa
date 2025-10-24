package main

import (
	"fmt"
	"sync"
)

type OptimalQueue[T any] struct {
	data []T
	head int // index of the first element
	tail int // index of the next free space
	size int // number of elements
	mu   sync.Mutex
}

func NewOptimalQueue[T any]() *OptimalQueue[T] {
	return &OptimalQueue[T]{
		data: make([]T, 0, 4), // initial capacity 4
		head: 0,
		tail: 0,
		size: 0,
	}
}

// Enqueue adds element to the end of the queue - O(1)
func (q *OptimalQueue[T]) Enqueue(element T) {
	q.mu.Lock()
	defer q.mu.Unlock()

	if q.size == len(q.data) {
		q.resize()
	}

	q.data[q.tail] = element
	q.tail = (q.tail + 1) % len(q.data)
	q.size++
}

// Dequeue removes element from the start of the queue - O(1)
func (q *OptimalQueue[T]) Dequeue() (T, bool) {
	q.mu.Lock()
	defer q.mu.Unlock()

	if q.size == 0 {
		var zero T
		return zero, false
	}

	element := q.data[q.head]
	q.head = (q.head + 1) % len(q.data)
	q.size--

	return element, true
}

// Peek returns the first element without removing it - O(1)
func (q *OptimalQueue[T]) Peek() (T, bool) {
	q.mu.Lock()
	defer q.mu.Unlock()

	if q.size == 0 {
		var zero T
		return zero, false
	}

	return q.data[q.head], true
}

// Size returns the number of elements - O(1)
func (q *OptimalQueue[T]) Size() int {
	q.mu.Lock()
	defer q.mu.Unlock()
	return q.size
}

// IsEmpty checks if the queue is empty - O(1)
func (q *OptimalQueue[T]) IsEmpty() bool {
	q.mu.Lock()
	defer q.mu.Unlock()
	return q.size == 0
}

// Clear clears the queue - O(1)
func (q *OptimalQueue[T]) Clear() {
	q.mu.Lock()
	defer q.mu.Unlock()
	q.head = 0
	q.tail = 0
	q.size = 0
}

// ToSlice returns a copy of all elements - O(n)
func (q *OptimalQueue[T]) ToSlice() []T {
	q.mu.Lock()
	defer q.mu.Unlock()

	if q.size == 0 {
		return []T{}
	}

	result := make([]T, q.size)
	for i := range q.size {
		index := (q.head + i) % len(q.data)
		result[i] = q.data[index]
	}
	return result
}

// resize increases the size of the array by 2 - O(n)
func (q *OptimalQueue[T]) resize() {
	newSize := len(q.data) * 2
	if newSize == 0 {
		newSize = 4
	}

	newData := make([]T, newSize)

	for i := range q.size {
		index := (q.head + i) % len(q.data)
		newData[i] = q.data[index]
	}

	q.data = newData
	q.head = 0
	q.tail = q.size
}

// TestOptimalQueue tests the optimal queue
func TestOptimalQueue() {
	fmt.Println("=== Test optimal queue ===")

	q := NewOptimalQueue[int]()

	fmt.Printf("Empty queue: isEmpty=%v, size=%d\n", q.IsEmpty(), q.Size())

	fmt.Println("\nAdd elements: 1, 2, 3, 4, 5")
	for i := range 5 {
		q.Enqueue(i)
	}

	fmt.Printf("Queue size: %d, isEmpty: %v\n", q.Size(), q.IsEmpty())
	fmt.Printf("All elements: %v\n", q.ToSlice())

	if val, ok := q.Peek(); ok {
		fmt.Printf("First element (peek): %d\n", val)
	}

	fmt.Println("\nRemove elements:")
	for range 3 {
		if val, ok := q.Dequeue(); ok {
			fmt.Printf("Removed element: %d\n", val)
		} else {
			fmt.Println("Queue is empty!")
		}
	}

	fmt.Printf("Remaining elements: %v\n", q.ToSlice())
	fmt.Printf("Final size: %d\n", q.Size())
}

// BenchmarkComparison compares the performance
func BenchmarkComparison() {
	fmt.Println("\n=== Performance comparison ===")

	n := 10000000

	optimalQ := NewOptimalQueue[int]()

	for i := range n {
		optimalQ.Enqueue(i)
	}

	for range n {
		optimalQ.Dequeue()
	}

	fmt.Printf("Optimal queue: %d operations completed successfully\n", n)
	fmt.Printf("Final size: %d\n", optimalQ.Size())
}
