package main

import (
	"fmt"
	"sync"
)

func main() {
	s := new(stack)

	s.Push(1)
	s.Push(2)
	s.Push(3)

	val, ok := s.Top()
	fmt.Println("Top:", val, "Success:", ok) // Top: 3 Exists: true
	empty := s.IsEmpty()
	fmt.Println("Is Empty:", empty) // Is Empty: false
	fmt.Println("Size:", s.Size())  // Size: 3
	pop, ok := s.Pop()
	fmt.Println("Popped:", pop, "Success:", ok) // Popped: 3 Success: true
	fmt.Println("Size after pop:", s.Size())    // Size after pop: 2
	s.Clear()
	fmt.Println("Size after clear:", s.Size()) // Size after clear: 0
}

type stack struct {
	mu   sync.Mutex
	data []int
}

type Stack interface {
	Push(value int)
	Pop() (int, bool)
	IsEmpty() bool
	Size() int
	Clear()
	Top() (int, bool)
}

func (s *stack) Push(value int) {
	s.mu.Lock()
	defer s.mu.Unlock()

	s.data = append(s.data, value)
}

func (s *stack) Pop() (int, bool) {
	s.mu.Lock()
	defer s.mu.Unlock()

	if len(s.data) == 0 {
		return 0, false
	}

	lastIndex := len(s.data) - 1
	lastValue := s.data[lastIndex]
	s.data = s.data[:lastIndex]

	return lastValue, true
}

func (s *stack) IsEmpty() bool {
	s.mu.Lock()
	defer s.mu.Unlock()

	return len(s.data) == 0
}

func (s *stack) Size() int {
	s.mu.Lock()
	defer s.mu.Unlock()

	return len(s.data)
}

func (s *stack) Clear() {
	s.mu.Lock()
	defer s.mu.Unlock()

	s.data = s.data[:0]
}

func (s *stack) Top() (int, bool) {
	s.mu.Lock()
	defer s.mu.Unlock()

	if len(s.data) == 0 {
		return 0, false
	}

	return s.data[len(s.data)-1], true
}
