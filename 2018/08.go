package main

import (
	"bytes"
	"container/list"
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"unicode"
)

type Node struct {
	children []*Node
	metadata []int
}

func main() {
	buffer, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}
	buffer = bytes.TrimFunc(buffer, unicode.IsSpace)
	var data []int
	for _, b := range bytes.Split(buffer, []byte(" ")) {
		s := string(b)
		i, err := strconv.Atoi(s)
		if err != nil {
			panic(err)
		}
		data = append(data, i)
	}
	pos := 0
	var parse_node func() *Node
	parse_node = func() *Node {
		nodes := data[pos]
		pos++
		metadata := data[pos]
		pos++
		node := &Node{nil, nil}
		for j := 0; j < nodes; j++ {
			child := parse_node()
			node.children = append(node.children, child)
		}
		for j := 0; j < metadata; j++ {
			node.metadata = append(node.metadata, data[pos])
			pos++
		}
		return node
	}
	root := parse_node()
	queue := list.New()
	queue.PushBack(root)
	sum := 0
	for queue.Len() > 0 {
		node_ := queue.Front()
		queue.Remove(node_)
		node := node_.Value.(*Node)
		for _, c := range node.children {
			queue.PushBack(c)
		}
		for _, m := range node.metadata {
			sum += m
		}
	}
	fmt.Println("Sum:", sum)
	sum = 0
	queue.PushBack(root)
	for queue.Len() > 0 {
		node_ := queue.Front()
		queue.Remove(node_)
		node := node_.Value.(*Node)
		for _, m := range node.metadata {
			if len(node.children) == 0 {
				sum += m
			} else {
				if m > 0 && m <= len(node.children) {
					queue.PushBack(node.children[m-1])
				}
			}
		}
	}
	fmt.Println("Sum 2:", sum)
}
