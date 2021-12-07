package main

import (
	"fmt"
)

func main() {
	var id, x, y, w, h int
	fabric := make(map[int]map[int]int)
	for {
		i, err := fmt.Scanf("#%d @ %d,%d: %dx%d\n", &id, &x, &y, &w, &h)
		if err != nil {
			if err.Error() == "unexpected EOF" {
				break
			}
			panic(err)
		}
		fmt.Println(i, id, x, y, w, h)
		for i := 0; i < w; i++ {
			if fabric[x+i] == nil {
				fabric[x+i] = make(map[int]int)
			}
			for j := 0; j < h; j++ {
				fabric[x+i][y+j]++
			}
		}
	}
	collisions := 0
	for _, fx := range fabric {
		for _, fy := range fx {
			if fy > 1 {
				collisions++
			}
		}
	}
	fmt.Println(collisions)
}
