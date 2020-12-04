package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	util "github.com/nickpegg/advent_of_code/2020/common"
)

func main() {
	data, err := util.ReadLines("input")
	if err != nil {
		panic(err)
	}
	fmt.Println("Result 1:", partOne(data))
	fmt.Println("Result 2:", partTwo(data))
}

func parse(line string) (int, int, string, string) {
	re := regexp.MustCompile(`(\d+)\-(\d+) (\w): (\w+)`)
	results := re.FindStringSubmatch(line)
	if len(results) != 5 {
		panic(fmt.Sprint("bad line:", line))
	}
	min, err := strconv.Atoi(results[1])
	if err != nil {
		panic(err)
	}
	max, err := strconv.Atoi(results[2])
	if err != nil {
		panic(err)
	}

	return min, max, results[3], results[4]
}

func partOne(data []string) int {
	validCount := 0
	for _, line := range data {
		min, max, char, password := parse(line)
		occurences := strings.Count(password, char)
		if occurences >= min && occurences <= max {
			validCount += 1
		}
	}

	return validCount
}

func partTwo(data []string) int {
	validCount := 0
	for _, line := range data {
		pos1, pos2, char, password := parse(line)
		char1 := string(password[pos1-1])
		char2 := string(password[pos2-1])
		if char1 != char2 && (char1 == char || char2 == char) {
			validCount += 1
		}
	}
	return validCount
}
