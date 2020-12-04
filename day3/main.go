package main

import (
	"fmt"
	"strings"

	util "github.com/nickpegg/advent_of_code/2020/common"
)

type traversal struct{
	Right int
	Down int
}

func main() {
	data, err := util.ReadLines("input")
	if err != nil { panic(err) }
	treeMap := parseMap(data)

	fmt.Println("Part 1:", partOne(treeMap))
	fmt.Println("Part 2:", partTwo(treeMap))
}

func partOne(treeMap [][]string) int {
	return traverse(treeMap, traversal{3, 1})
}

func partTwo(treeMap [][]string) int {
	result := 1
	steps := []traversal{
		traversal{1, 1},
		traversal{3, 1},
		traversal{5, 1},
		traversal{7, 1},
		traversal{1, 2},
	}

	for _, step := range(steps) {
		trees := traverse(treeMap, step)
		result *= trees
	}
	return result
}

// parseMap turns an array of strings into a 2D array of single-character strings. First coordiate
// is Y, second is X
func parseMap(tree_map []string) [][]string {
	newMap := make([][]string, len(tree_map))
	for i, s := range(tree_map) {
		newMap[i] = strings.Split(s, "")
	}
	return newMap
}

// traverse traverses the map, starting at 0,0 (upper left corner) looking for trees
func traverse(treeMap [][]string, t traversal) int {
	treeCount := 0
	pos_x := 0
	pos_y := 0

	height := len(treeMap)
	width := len(treeMap[0])

	for pos_y < height {
		if treeMap[pos_y][pos_x] == "#" {
			treeCount++
		}

		pos_x += t.Right
		pos_y += t.Down

		if pos_x >= width {
			// went past right edge, loop around
			pos_x -= width
		}
	}
	return treeCount
}
