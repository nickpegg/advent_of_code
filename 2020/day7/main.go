package main

import (
	"fmt"
	util "github.com/nickpegg/advent_of_code/2020/common"
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
	bm, err := NewBagMap(lines)
	if err != nil {
		panic(err)
	}
	bg, err := NewBagGraph(bm)
	if err != nil {
		panic(err)
	}

	colors := bg.Contains("shiny gold")
	return len(colors)
}

func partTwo(lines []string) int {
	// How many bags do you need total for one shiny gold bag
	bm, err := NewBagMap(lines)
	if err != nil {
		panic(err)
	}

	// Subtract 1 since we're not counting the shiny gold bag itself
	return bm.TotalBags("shiny gold") - 1
}
