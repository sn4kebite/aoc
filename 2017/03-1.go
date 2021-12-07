package main

import (
	"fmt"
	"io/ioutil"
	"os"
	//"math"
	"strconv"
	"strings"
	"unicode"
)

func min(a int, b int) int {
	if a < b {
		return a
	} else {
		return b
	}
}

func abs(v int) int {
	if v < 0 {
		return -v
	} else {
		return v
	}
}

func spiral(level int, num int, last_max int) int {
	size := 2*level
	max := last_max + size * 4
	center := last_max + level
	fmt.Printf("level=%v last_max=%v size=%v max=%v\n", level, last_max, size, max)
	fmt.Printf("  center: %v %v %v %v\n", center+size*0, center+size*1, center+size*2, center+size*3)
	if num > max {
		return 1 + spiral(level + 1, num, max)
	}
	distance :=
	min(
		min(
			min(
				min(size, abs(center - num)),
				abs((center+size*1) - num)),
			abs((center+size*2) - num)),
		abs((center+size*3) - num))
	fmt.Printf("  distance=%v\n", distance)
	return 1 + distance
}

func spiral1(num int) int {
	distance := 1
	if num > 9 {
		distance += spiral(2, num, 9)
	} else {
		distance += num % 2
	}
	return distance
}

func spiral0(num int) int {
	if num <= 1 {
		return 0
	}
	return spiral1(num)
}

func main() {
	buffer, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
		return
	}
	d, err := strconv.Atoi(strings.TrimFunc(string(buffer), unicode.IsSpace))
	if err != nil {
		panic(err)
		return
	}
	fmt.Printf("distance: %v\n", spiral0(d))
}
