package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

var lines = []string{
	"1-3 a: abcde",
	"1-3 b: cdefg",
	"2-9 c: ccccccccc",
}

func TestPartOne(t *testing.T) {
	require.Equal(t, partOne(lines), 2)
}

func TestPartTwo(t *testing.T) {
	require.Equal(t, partTwo(lines), 1)
}
