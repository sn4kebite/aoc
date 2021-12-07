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
	var rows [][]int
	for scanner.Scan() {
		str := strings.Split(scanner.Text(), "\t")
		var row []int
		for _, s := range str {
			value, err := strconv.Atoi(s)
			if err != nil {
				fmt.Println(err)
				return
			}
			row = append(row, value)
		}
		rows = append(rows, row)
	}
	for _, row := range rows {
		fmt.Printf("row with %v values\n", len(row))
		for i := 0; i < len(row)-1; i++ {
			for j := i+1; j < len(row); j++ {
				if row[i] % row[j] == 0 {
					checksum += row[i] / row[j]
					break
				}
				if row[j] % row[i] == 0 {
					checksum += row[j] / row[i]
					break
				}
			}
		}
	}
	fmt.Println(checksum)
}
