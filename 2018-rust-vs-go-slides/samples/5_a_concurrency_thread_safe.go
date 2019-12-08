package main

import (
	"fmt"
)

func main() {
	channel := make(chan struct{})

	gopherCounter := 0
	// m := map[string]string{}

	// start 1000 go routines and increment the counter
	for i := 0; i < 1000; i += 1 {
		go func() {
			gopherCounter++
			// m["uhoh"] = "picnic"

			channel <- struct{}{}
		}()
	}

	// read from channel 1000 times so main thread does exit before routines
	for i := 0; i < 1000; i += 1 {
		<-channel
	}

	fmt.Println(gopherCounter) // ???
}
