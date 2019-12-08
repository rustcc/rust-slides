package main

import (
	"fmt"
)

// cannot specify immutability of struct fields
type meetup struct {
	date      string
	attendees []string
}

func main() {
	gophers := make([]string, 0, 3)
	gophers = append(gophers, "ann", "sam", "bob")
	today := meetup{"2018-08-21", gophers}

	next := today // easy implicit copy of struct
	mutateStruct(&next)

	fmt.Printf("Curr Meetup: %v\n", today)
	fmt.Printf("Next Meetup: %v\n", next)

	sometimesMutate(today.attendees)

	fmt.Printf("Curr Meetup: %v\n", today)
}

// except things like array, map, slice are passed by ref
func mutateStruct(meetup *meetup) {
	meetup.date = "2018-09-28"
	meetup.attendees[0] = "ferris"
	meetup.attendees[1] = "tom"
}

// effects of appending to subslice varies depending on cap of orig slice
// try with cap 4 slice
func sometimesMutate(gophers []string) {
	attendees := gophers[:]
	attendees = append(attendees, "am")
	attendees[0] = "gopher"
}
