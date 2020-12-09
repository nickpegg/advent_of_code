package main

import (
	"fmt"
	util "github.com/nickpegg/advent_of_code/2020/common"
	"strconv"
	"strings"
)

type Machine struct {
	Program []Instruction

	Accumulator    int
	ProgramCounter int
}

type Instruction struct {
	Name  string
	Value int
}

// RunNoLoop execute the stored program, stopping if a loop is detected
func (m *Machine) Run() error {
	pcValues := make(map[int]bool)

	for m.ProgramCounter = 0; m.ProgramCounter < len(m.Program); m.ProgramCounter++ {
		_, seen := pcValues[m.ProgramCounter]
		if seen {
			return fmt.Errorf("Infinite Loop")
		}
		pcValues[m.ProgramCounter] = true

		instruction := m.Program[m.ProgramCounter]
		switch instruction.Name {
		case "nop":
			break
		case "acc":
			m.Accumulator += instruction.Value
		case "jmp":
			m.ProgramCounter += instruction.Value - 1
		default:
			return fmt.Errorf("Invalid instruction: %v", instruction)
		}
	}

	return nil
}

// LoadScript reads a machine script from a text file, one instruction per line
func LoadScript(filename string) ([]Instruction, error) {
	instructions := make([]Instruction, 0)

	lines, err := util.ReadLines(filename)
	if err != nil {
		return nil, err
	}

	for _, line := range lines {
		parts := strings.Split(line, " ")
		if len(parts) != 2 {
			return nil, fmt.Errorf("Invalid instruction: %s", line)
		}

		value, err := strconv.Atoi(parts[1])
		if err != nil {
			return nil, fmt.Errorf("Invalid value in instructions: %s", line)
		}
		instructions = append(instructions, Instruction{parts[0], value})
	}
	return instructions, nil
}
