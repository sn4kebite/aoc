package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
)

func main() {
	//scanner := bufio.NewScanner(os.Stdin)
	buffer, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		fmt.Println(err)
		return
	}
	value := 0
	frequencies := make(map[int]bool)
	frequencies[0] = true
	for {
		scanner := bufio.NewScanner(bytes.NewReader(buffer))
		for scanner.Scan() {
			i, err := strconv.Atoi(scanner.Text())
			if err != nil {
				fmt.Println(nil)
			}
			value += i
			//fmt.Printf("i=%v value=%v exists=%v\n", i, value, frequencies[value])
			if frequencies[value] {
				fmt.Println(value)
				return
			}
			frequencies[value] = true
		}
	}
}
