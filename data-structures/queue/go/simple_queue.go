package main

import (
	"fmt"
	"sync"
)

type queue[T any] struct {
	data []T
	mu   sync.Mutex
}

type Queue[T any] interface {
	Enqueue(element T)  // FIFO: add to the end of queue
	Dequeue() (T, bool) // FIFO: remove from the start of the queue
	Peek() (T, bool)    // FIFO: get first element of the queue
	Size() int          // FIFO: get size of the queue
	IsEmpty() bool      // FIFO: is the queue is empty
	Clear()             // FIFO: clears the queue
}

func newQueue[T any]() queue[T] {
	return queue[T]{
		data: make([]T, 0),
	}
}

func (q *queue[T]) Enqueue(element T) {
	q.mu.Lock()
	defer q.mu.Unlock()
	q.data = append(q.data, element)
}

func (q *queue[T]) Dequeue() (T, bool) {
	q.mu.Lock()
	defer q.mu.Unlock()

	if len(q.data) == 0 {
		var zero T
		return zero, false
	}

	firstElement := q.data[0]
	q.data = q.data[1:]

	return firstElement, true
}

func (q *queue[T]) Peek() (T, bool) {
	q.mu.Lock()
	defer q.mu.Unlock()

	if len(q.data) == 0 {
		var zero T
		return zero, false
	}

	return q.data[0], true
}

func (q *queue[T]) Size() int {
	q.mu.Lock()
	defer q.mu.Unlock()

	return len(q.data)
}

func (q *queue[T]) IsEmpty() bool {
	q.mu.Lock()
	defer q.mu.Unlock()

	return len(q.data) == 0
}

func (q *queue[T]) Clear() {
	q.mu.Lock()
	defer q.mu.Unlock()
	q.data = q.data[:0]
}

func testSimpleQueue() {
	q := newQueue[int]()
	fmt.Printf("Empty queue: isEmpty=%v, size=%d\n", q.IsEmpty(), q.Size())

	fmt.Println("\nAdd elements: 1, 2, 3")
	q.Enqueue(1)
	q.Enqueue(2)
	q.Enqueue(3)

	fmt.Printf("Queue size: %d, isEmpty: %v\n", q.Size(), q.IsEmpty())

	if val, ok := q.Peek(); ok {
		fmt.Printf("First element (peek): %d\n", val)
	}

	fmt.Println("\nRemove elements:")
	for range 4 {
		if val, ok := q.Dequeue(); ok {
			fmt.Printf("Removed element: %d\n", val)
		} else {
			fmt.Println("Queue is empty!")
		}
	}

	fmt.Printf("Final size: %d\n", q.Size())
}
