package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	last_c := byte(0)
	first_c := byte(0)
	total := 0
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanBytes)
	for scanner.Scan() {
		s := scanner.Text()
		if len(s) != 1 {
			fmt.Println("input should be 1 byte!")
			return
		}
		c := s[0]
		if c < '0' || c > '9' {
			continue
		}
		d := int(c - '0')
		if first_c == 0 {
			first_c = c
		}
		if c == last_c {
			total += d
		}
		last_c = c
	}
	if last_c == first_c {
		total += int(last_c - '0')
	}
	fmt.Println(total)
}
