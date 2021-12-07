package main

import (
	"bufio"
	//"bytes"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	checksum := 0
	for scanner.Scan() {
		low := 0
		high := 0
		for _, s := range strings.Split(scanner.Text(), "\t") {
			value, err := strconv.Atoi(s)
			if err != nil {
				fmt.Println(err)
				return
			}
			if low == 0 && high == 0 {
				low = value
				high = value
			}
			if value < low {
				low = value
			}
			if value > high {
				high = value
			}
		}
		checksum += high - low
	}
	fmt.Println(checksum)
}
