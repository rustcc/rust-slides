package main

import (
	"fmt"
	"sort"
)

// filter, map, dedup, sort, take first 3, sum...
func reallyContrivedExample(xs []int) int {
	m := map[int]struct{}{}
	temp := []int{}
	sum := 0

	for _, v := range xs {
		if v >= 10 {
			m[v+1] = struct{}{}
		}
	}

	for k := range m {
		temp = append(temp, k)
	}

	sort.Ints(temp)

	for i, v := range temp {
		sum += v

		if i == 2 {
			break
		}
	}

	return sum
}

func main() {
	nums := []int{1, 2, 10, 13, 16, 40, 50, 60}
	fmt.Println(reallyContrivedExample(nums))
}
