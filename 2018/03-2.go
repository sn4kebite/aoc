package main

import (
	"fmt"
)

func main() {
	var id, x, y, w, h int
	fabric := make(map[int]map[int][]int)
	ids := make(map[int]int)
	for {
		i, err := fmt.Scanf("#%d @ %d,%d: %dx%d\n", &id, &x, &y, &w, &h)
		if err != nil {
			if err.Error() == "unexpected EOF" {
				break
			}
			fmt.Printf("%T\n", err)
			panic(err)
		}
		fmt.Println(i, id, x, y, w, h)
		ids[id] = w*h
		for i := 0; i < w; i++ {
			if fabric[x+i] == nil {
				fabric[x+i] = make(map[int][]int)
			}
			for j := 0; j < h; j++ {
				fabric[x+i][y+j] = append(fabric[x+i][y+j], id)
			}
		}
	}
	for _, fx := range fabric {
		for _, fy := range fx {
			if len(fy) > 1 {
				continue
			}
			for _, id := range fy {
				ids[id]--
				if ids[id] == 0 {
					fmt.Printf("ID %v has no collisions!\n", id)
					return
				}
			}
		}
	}
}
