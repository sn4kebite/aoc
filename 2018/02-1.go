package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	two := 0
	three := 0
	for scanner.Scan() {
		chars := make(map[rune]int)
		for _, c := range scanner.Text() {
			chars[c]++
		}
		has_two := false
		has_three := false
		for _, v := range chars {
			if v == 2 {
				has_two = true
			}
			if v == 3 {
				has_three = true
			}
		}
		if has_two {
			two++
		}
		if has_three {
			three++
		}
	}
	fmt.Println(two * three)
}
