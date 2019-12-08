package main

import (
	"fmt"
	"strings"
)

// compare all ints and return the max
func maxInts(xs []int) int {
	max := xs[0] // panic?

	for _, v := range xs {
		if v > max {
			max = v
		}
	}
	return max
}

// compare all strings lexicographically and return the max
func maxStrings(xs []string) string {
	max := xs[0] // panic?

	for _, v := range xs {
		if strings.Compare(v, max) == 1 {
			max = v
		}
	}
	return max
}

func main() {
	someInts := []int{3, 4, 1, 7, 8, 324}
	someStrings := []string{"cat", "dog", "horse", "sleeping gopher"}

	fmt.Printf("%d %s\n", maxInts(someInts), maxStrings(someStrings))
}
