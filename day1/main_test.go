package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

var nums = []int{1721, 979, 366, 299, 675, 1456}

func TestPartOne(t *testing.T) {
	result := partOne(nums)
	require.Equal(t, result, 514579)
}

func TestPartTwo(t *testing.T) {
	result := partTwo(nums)
	require.Equal(t, result, 241861950)
}
