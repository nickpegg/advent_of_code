package main

import (
	"github.com/stretchr/testify/require"
	"testing"
)

func TestPartOne(t *testing.T) {
	groups := loadFile("test_input")
	require.Equal(t, 11, partOne(groups))
}

func TestPartTwo(t *testing.T) {
	groups := loadFile("test_input")
	require.Equal(t, 6, partTwo(groups))
}
