package main

import (
	"bufio"
	"container/list"
	"fmt"
	"os"
)

type Rule struct {
	a, b string
}

type Step struct {
	value string
	deps []*Step
	queued, done bool
	working bool
	remaining int
}

func NewStep(value string) *Step {
	return &Step{value, nil, false, false, false, int(value[0] - 'A' + 1 + 60)}
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	mm := make(map[string]*Step)
	var steps []*Step
	queue := list.New()
	for scanner.Scan() {
		var before_str, after_str string
		fmt.Sscanf(scanner.Text(), "Step %s must be finished before step %s can begin.", &before_str, &after_str)
		b := mm[before_str]
		if b == nil {
			b = NewStep(before_str)
			mm[before_str] = b
			queue.PushBack(b)
		}
		a := mm[after_str]
		if a == nil {
			a = NewStep(after_str)
			mm[after_str] = a
			queue.PushBack(a)
		}
		a.deps = append(a.deps, b)
	}
	time := 0
	var working []*Step
	const workers = 2
	advance_time := func(t int) {
		if len(working) == 0 {
			return
		}
		time += t
		fmt.Println("Tick", time)
		for i := 0; i < len(working); i++ {
			s := working[i]
			s.remaining -= t
			if s.remaining <= 0 {
				fmt.Println("  Step ", s, "is done")
				s.remaining = 0
				s.done = true
				s.working = false
				working = append(working[:i], working[i+1:]...)
				i--
			}
		}
	}
	for queue.Len() > 0 {
		advance_time(1)
		for s := queue.Front(); s != nil; s = s.Next() {
			step := s.Value.(*Step)
			pending_deps := 0
			for _, d := range step.deps {
				if ! d.done {
					pending_deps++
				}
			}
			if pending_deps == 0 && ! step.working && ! step.done {
				prev := s.Prev()
				fmt.Printf("removing %v\n", s)
				queue.Remove(s)
				if prev == nil {
					s = queue.Front()
				} else {
					s = prev
				}
				step.working = true
				working = append(working, step)
				steps = append(steps, step)
				fmt.Printf("node %v\n", step)
			}
			if s == nil {
				break
			}
		}
	}
	for len(working) > 0 {
		advance_time(1)
	}
	fmt.Print("Order: ")
	for _, s := range steps {
		fmt.Print(s.value)
	}
	fmt.Println()
	fmt.Println("Time:", time)
}
