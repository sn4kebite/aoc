package main

import (
	"bufio"
	"fmt"
	"os"
	"time"
)

type Vector struct {
	x, y int
}

type Point struct {
	pos Vector
	vel Vector
}

func (v Vector) Add(o Vector) Vector {
	return Vector{v.x + o.x, v.y + o.y}
}

func main() {
	var points []*Point
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		px, py, vx, vy := 0, 0, 0, 0
		fmt.Sscanf(scanner.Text(), "position=<%d, %d> velocity=<%d, %d>", &px, &py, &vx, &vy)
		points = append(points, &Point{Vector{px, py}, Vector{vx, vy}})
	}
	seconds := 0
	for {
		min_x :=  9999
		min_y :=  9999
		max_x := -9999
		max_y := -9999
		for _, p := range points {
			p.pos = p.pos.Add(p.vel)
			if p.pos.x < min_x {
				min_x = p.pos.x
			}
			if p.pos.x > max_x {
				max_x = p.pos.x
			}
			if p.pos.y < min_y {
				min_y = p.pos.y
			}
			if p.pos.y > max_y {
				max_y = p.pos.y
			}
		}
		seconds++
		spread := (max_x - min_x) + (max_y - min_y)
		if spread > 100 {
			continue
		}
		fmt.Print("\x1B[H")
		fmt.Print("\x1B[2J")
		for y := min_y; y <= max_y; y++ {
			for x := min_x; x <= max_x; x++ {
				point := false
				for _, p := range points {
					if p.pos.x == x && p.pos.y == y {
						point = true
						break
					}
				}
				if point {
					fmt.Print("#")
				} else {
					fmt.Print(" ")
				}
			}
			fmt.Println()
		}
		fmt.Println("Seconds:", seconds)
		time.Sleep(1 * time.Second)
	}
}
