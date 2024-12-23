package main

import (
	"testing"

	util "github.com/nickpegg/advent_of_code/2020/common"
	"github.com/stretchr/testify/require"
)

func TestPartOne(t *testing.T) {
	lines, err := util.ReadLines("test_input")
	if err != nil {
		t.Fatal(err)
	}

	result := partOne(lines)
	require.Equal(t, 4, result)
}

func TestPartTwo(t *testing.T) {
}
