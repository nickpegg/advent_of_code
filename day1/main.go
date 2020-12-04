package main

import (
	"fmt"
	util "github.com/nickpegg/advent_of_code/2020/common"
	"strconv"
)

func main() {
	data, err := util.ReadLines("input")
	if err != nil {
		panic(err)
	}

	// Turn the list of strings into ints
	numbers := make([]int, len(data))
	for i, s := range data {
		if s == "" {
			continue
		}

		number, err := strconv.Atoi(s)
		if err != nil {
			panic(err)
		}

		numbers[i] = number
	}
	fmt.Println(numbers)

	result1 := partOne(numbers)
	if result1 == 0 {
		fmt.Println("no result for part 1")
	} else {
		fmt.Println("Result 1:", result1)
	}

	fmt.Println("Result 2:", partTwo(numbers))
}

func partOne(nums []int) int {
	sumTo := 2020
	jawns := make(map[int]bool)

	for _, num := range(nums) {
		candidate := sumTo - num
		_, exists := jawns[candidate]
		if exists {
			return num * candidate
		}
		jawns[num] = true
	}

	return 0
}

func partTwo(nums []int) int {
	// Fukkit, brute force time
	for _, i := range(nums) {
		for _, j := range(nums) {
			for _, k := range(nums) {
				if i + j + k == 2020 {
					return i * j * k
				}
			}
		}
	}
	return 0
}
