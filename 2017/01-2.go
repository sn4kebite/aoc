package main

import (
	"bytes"
	"fmt"
	//"io"
	"io/ioutil"
	"os"
	"unicode"
)

func main() {
	total := 0
	//buffer := make([]byte, 10240)
	buffer, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}
	buffer = bytes.TrimFunc(buffer, unicode.IsSpace)
	//var buffer bytes.Buffer
	/*for {
		tmpbuf := make([]byte, 1024)
		_size, err := os.Stdin.Read(tmpbuf)
		tmpbuf = bytes.TrimFunc(tmpbuf, unicode.IsNumber)
		buffer.Write(tmpbuf)
		fmt.Println("read", _size, "bytes")
		if err == io.EOF {
			break
		}
		if err != nil {
			fmt.Println(err)
			return
		}
	}*/
	size := len(buffer)
	for i := 0; i < size; i++ {
		c := buffer[i]
		if c < '0' || c > '9' {
			continue
		}
		d := int(c - '0')
		if c == buffer[(i + size / 2) % size] {
			total += d
		}
	}
	fmt.Println(total)
}
