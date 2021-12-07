package main

import (
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
	recipes := []int{3, 7}
	for len(recipes) < max_recipes+10 {
		sum := recipes[elf1] + recipes[elf2]
		for j := int(math.Log10(float64(sum))); j > 0; j-- {
			recipes = append(recipes, sum / (j * 10))
		}
		recipes = append(recipes, sum % 10)
		elf1 = (elf1 + 1 + recipes[elf1]) % len(recipes)
		elf2 = (elf2 + 1 + recipes[elf2]) % len(recipes)
		/*for j, r := range recipes {
			if j == elf1 {
				fmt.Printf("(%d) ", r)
			} else if j == elf2 {
				fmt.Printf("[%d] ", r)
			} else {
				fmt.Printf("%d ", r)
			}
		}
		fmt.Println()*/
	}
	fmt.Print("Scores: ")
	for i := max_recipes; i < max_recipes+10; i++ {
		fmt.Print(recipes[i])
	}
	fmt.Println()
}
