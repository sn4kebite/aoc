package main

import (
	"fmt"
	"os"
	"strconv"
)

func max(a, b int) int {
	if a > b {
		return a
	} else {
		return b
	}
}

func main() {
	serial, err := strconv.Atoi(os.Args[1])
	if err != nil {
		panic(err)
	}
	fmt.Println("Serial:", serial)
	var cells [300*300]int
	for x := 0; x < 300; x++ {
		for y := 0; y < 300; y++ {
			rack_id := x + 11
			value := (rack_id * (y + 1) + serial) * rack_id
			value = (value / 100) % 10 - 5
			cells[y*300+x] = value
		}
	}
	var largest_value, largest_x, largest_y, largest_size int
	for x := 0; x < 300; x++ {
		for y := 0; y < 300; y++ {
			value := 0
			for size := 0; size < 300-max(x, y); size++ {
				for i := 0; i < size; i++ {
					value += cells[(y+size) * 300 + x+i]
					value += cells[(y+i) * 300 + x+size]
				}
				value += cells[(y+size) * 300 + x+size]
				if value > largest_value {
					largest_value = value
					largest_x = x
					largest_y = y
					largest_size = size
				}
			}
		}
	}
	fmt.Printf("Largest: %d at %d,%d,%d\n", largest_value, largest_x+1, largest_y+1, largest_size+1)
}
