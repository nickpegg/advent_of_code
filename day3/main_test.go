package main

import (
	"testing"

	"github.com/stretchr/testify/require"

	util "github.com/nickpegg/advent_of_code/2020/common"
)

func getTestInput(t *testing.T) [][]string {
	data, err := util.ReadLines("test_input")
	if err != nil { t.Fatal(err) }
	return parseMap(data)
}

func testParseMap(t *testing.T) {
	in := []string{"abcde", "efg"}
	out := [][]string{
		[]string{"a", "b", "c", "d", "e"},
		[]string{"e", "f", "g"},
	}

	require.Equal(t, parseMap(in), out)
}

func TestTraverse(t *testing.T) {
	in := getTestInput(t)
	require.Equal(t, traverse(in, traversal{3, 1}), 7)
}

func TestPartTwo(t *testing.T) {
	in := getTestInput(t)
	require.Equal(t, partTwo(in), 336)
}
