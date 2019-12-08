package main

import (
	"fmt"
)

type Gopher struct {
	name string
}

func main() {
	if gopher, err := findGopher(); err != nil {
		fmt.Printf("Errors are values. [%#v], [%#v]\n", gopher, err)
	}

	// gopher, _ := findGopher()
	// fmt.Println(gopher.name)
}

func findGopher() (*Gopher, error) {
	return nil, fmt.Errorf("Could not find any gophers, must be hibernating.")
}
