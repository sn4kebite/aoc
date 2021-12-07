package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"unicode"
)

func reduce(input []byte) (output []byte) {
	for i := 0; i < len(input); i++ {
		last_c := byte(0)
		if len(output) > 0 {
			last_c = output[len(output)-1]
		}
		if input[i] - last_c == 32 || last_c - input[i] == 32 {
			output = append(output[:len(output)-1])
			continue
		}
		output = append(output, input[i])
	}
	return output
}

func main() {
	buffer, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}
	buffer = bytes.TrimFunc(buffer, unicode.IsSpace)
	output := reduce(buffer)
	fmt.Printf("Size: %d\n", len(output))

	min_size := len(buffer)
	for c := 'A'; c <= 'Z'; c++ {
		f := func(r rune) rune {
			switch r {
				case c: return -1
				case c+32: return -1
			}
			return r
		}
		output := reduce(bytes.Map(f, buffer))
		l := len(output)
		if l < min_size {
			min_size = l
		}
	}
	fmt.Printf("Min size: %d\n", min_size)
}
