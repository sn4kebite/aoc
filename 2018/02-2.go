package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var ids []string
	for scanner.Scan() {
		ids = append(ids, scanner.Text())
	}
	for i := 0; i < len(ids)-1; i++ {
		for j := i+1; j < len(ids); j++ {
			diffs := 0
			var letters []byte
			for c := 0; c < len(ids[i]); c++ {
				if ids[i][c] != ids[j][c] {
					diffs++
					if diffs > 1 {
						break
					}
				} else {
					letters = append(letters, ids[i][c])
				}
			}
			if diffs == 1 {
				for _, l := range letters {
					fmt.Printf("%c", l)
				}
				fmt.Println()
				return
			}
		}
	}
}
