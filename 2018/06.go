package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

type Area struct {
	x, y int
	size int
}

func abs(v int) int {
	if v < 0 {
		return -v
	} else {
		return v
	}
}

func main() {
	var coords []*Area
	scanner := bufio.NewScanner(os.Stdin)
	min_x := math.MaxUint32
	max_x := 0
	min_y := math.MaxUint32
	max_y := 0
	for scanner.Scan() {
		var x, y int
		fmt.Sscanf(scanner.Text(), "%d, %d", &x, &y)
		if min_x > x {
			min_x = x
		}
		if max_x < x {
			max_x = x
		}
		if min_y > y {
			min_y = y
		}
		if max_y < y {
			max_y = y
		}
		coords = append(coords, &Area{x, y, 0})
	}
	max_dist := 10000
	region_size := 0
	for x := min_x; x <= max_x; x++ {
		for y := min_y; y <= max_y; y++ {
			dist := math.MaxUint32
			var closest *Area
			reg_dist := 0
			for _, c := range coords {
				c_dist := abs(c.x - x) + abs(c.y - y)
				reg_dist += c_dist
				if c_dist < dist {
					dist = c_dist
					closest = c
				}
			}
			if closest == nil {
				panic("no closest found")
			}
			closest.size++
			if reg_dist < max_dist {
				region_size++
			}
		}
	}
	var largest *Area = nil
	for _, c := range coords {
		if largest == nil || c.size > largest.size {
			largest = c
		}
	}
	if largest == nil {
		panic("no largest found")
	}
	fmt.Println("Largest:", largest)
	fmt.Println("Region size:", region_size)
}
