package main

import (
	"bufio"
	"fmt"
	"os"
)

type Rule struct {
	input string
	output rune
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	s := scanner.Text()
	pots := "..." + s[15:] + "..."
	scanner.Scan()
	var rules []Rule
	for scanner.Scan() {
		var input string
		var output rune
		fmt.Sscanf(scanner.Text(), "%s => %c", &input, &output)
		rules = append(rules, Rule{input, output})
	}
	fmt.Println("000:", pots)
	first_index := -3
	//iterations := 20
	iterations := 50000000000
	last_pots := pots
	for g := 0; g < iterations; g++ {
		//if g % 1000 == 0 {
		//	fmt.Printf("\r%3d %% %d", g * 100 / 50000000000, g)
		//}
		output := []rune(pots)
		for _, rule := range rules {
			for i := 2; i < len(pots) - 2; i++ {
				if pots[i-2:i+3] == rule.input {
					//fmt.Printf("'%s' matches rule '%s' => %c\n", pots[i-2:i+3], rule.input, rule.output)
					output[i] = rule.output
				}
			}
		}
		first := len(output)
		last := 0
		for i := 0; i < len(output); i++ {
			if output[i] == '#' {
				if first > i {
					first = i
				}
				if last < i {
					last = i
				}
			}
		}
		//fmt.Printf("first=%d last=%d\n", first, last)
		increment := 0
		for first < 3 {
			//fmt.Println("prepend ", first)
			output = append([]rune{'.'}, output...)
			first++
			last++
			first_index--
			increment++
		}
		//fmt.Println("increment ", increment)
		//first_index += increment
		for last+3 >= len(output) {
			output = append(output, '.')
		}
		if first > 3 {
			first_index += first-3
			increment += first-3
		}
		pots = string(output[first-3:last+4])
		fmt.Printf("%03d: ", g+1)
		//for i := first_index; i < -3; i++ {
		//	fmt.Printf(".")
		//}
		fmt.Print(pots)
		fmt.Println(" ", first_index)
		if last_pots == pots {
			fmt.Println("Stable on generation", g+1)
			fmt.Println(increment, first, last, first_index)
			//remaining = iterations-g-1
			first_index += increment * (iterations - g - 1)
			break
		}
		last_pots = pots
	}
	value := 0
	fmt.Println("first_index:", first_index)
	for i, c := range pots {
		if c == '#' {
			value += i + first_index
		}
	}
	fmt.Println("Value:", value)
}
