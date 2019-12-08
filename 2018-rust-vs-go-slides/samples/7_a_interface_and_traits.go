package main

import (
	"fmt"
)

type Gopher struct {
	name   string
	height float64
	weight float64
}

type Fighter interface {
	Stats() string
}

func (g *Gopher) Stats() string {
	return fmt.Sprintf("Name: %#s, height: %.2f, weight: %.2f", g.name, g.height, g.weight)
}

func fight(f Fighter) {
	fmt.Println(f.Stats())
}

func main() {
	tom := &Gopher{"tom", 16, 120}
	fight(tom)
}
