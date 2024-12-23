package main

import (
	"fmt"
	"strings"

	util "github.com/nickpegg/advent_of_code/2020/common"
)

const (
	planeRows = 128
	planeCols = 8
)

func main() {
	lines, err := util.ReadLines("input")
	if err != nil {
		panic(err)
	}

	fmt.Println("Part 1:", partOne(lines))
	fmt.Println("Part 2:", partTwo(lines))
}

func partOne(lines []string) int {
	// Find highest seat ID
	highestSid := 0

	for _, line := range lines {
		sid := walk(line)
		if sid > highestSid {
			highestSid = sid
		}
	}

	return highestSid
}

func partTwo(lines []string) []int {
	// Find seat ID, which is missing from the list. Seats with IDs +1 and -1 from ours will exist
	// though.

	// All sids we've seen from the list
	sids := make(map[int]bool)
	candidates := make([]int, 0)

	for r := 0; r < planeRows; r++ {
		for c := 0; c < planeCols; c++ {
			sid := SeatID(r, c)
			sids[sid] = false
		}
	}

	for _, line := range lines {
		sid := walk(line)
		sids[sid] = true
	}

	// okay, find the seat
	for sid, exists := range sids {
		if !exists && sids[sid-1] && sids[sid+1] {
			candidates = append(candidates, sid)
		}
	}

	return candidates
}

// walk goes through the rules and returns the Seat ID that was determined. Panics on error.
func walk(rules string) int {
	var err error

	zone := NewZone(planeRows, planeCols)
	for _, rule := range strings.TrimSpace(rules) {
		zone, err = zone.Bisect(rule)
		if err != nil {
			panic(err)
		}
	}
	row, col, err := zone.Seat()
	if err != nil {
		panic(err)
	}
	sid := SeatID(row, col)
	return sid
}
