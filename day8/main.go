package main

import (
	"fmt"
)

func main() {
	instructions, err := LoadScript("input")
	if err != nil {
		panic(err)
	}

	fmt.Println("Part 1:", partOne(instructions))
	fmt.Println("Part 2:", partTwo(instructions))
}

func partOne(ins []Instruction) int {
	m := &Machine{Program: ins}
	m.Run()
	return m.Accumulator
}

// partTwo tries swapping NOPs for JMPs until the program successfully terminates, and returns the
// value of the accumulator
func partTwo(ins []Instruction) int {
	for i, _ := range ins {
		if ins[i].Name == "nop" {
			ins[i].Name = "jmp"
			m := &Machine{Program: ins}
			err := m.Run()
			if err == nil {
				return m.Accumulator
			}
			ins[i].Name = "nop"
		} else if ins[i].Name == "jmp" {
			ins[i].Name = "nop"
			m := &Machine{Program: ins}
			err := m.Run()
			if err == nil {
				return m.Accumulator
			}
			ins[i].Name = "jmp"
		}
	}

	fmt.Println("Exhausted tries")
	return -1
}
