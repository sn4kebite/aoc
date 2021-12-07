package main

import (
	"bytes"
	"fmt"
	"math"
	"os"
	"strconv"
)

func main() {
	max_recipes, err := strconv.Atoi(os.Args[1])
	if err != nil {
		panic(err)
	}
	fmt.Println("Input:", max_recipes)
	elf1, elf2 := 0, 1
	recipes := []byte{3, 7}
	pattern := []byte(os.Args[1])
	for i, _ := range pattern {
		pattern[i] -= '0'
	}
	last := 0
	for bytes.Index(recipes[last:], pattern) == -1 {
		if len(recipes) > len(pattern) {
			last = len(recipes) - len(pattern)
		}
		for i := 0; i < 1000; i++ {
			sum := int(recipes[elf1] + recipes[elf2])
			for j := int(math.Log10(float64(sum))); j > 0; j-- {
				recipes = append(recipes, byte(sum / (j * 10)))
			}
			recipes = append(recipes, byte(sum % 10))
			elf1 = (elf1 + 1 + int(recipes[elf1])) % len(recipes)
			elf2 = (elf2 + 1 + int(recipes[elf2])) % len(recipes)
		}
	}
	fmt.Println(bytes.Index(recipes, pattern))
}
