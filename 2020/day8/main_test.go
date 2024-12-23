package main

import (
	"github.com/stretchr/testify/require"
	"testing"
)

func TestPartTwo(t *testing.T) {
	inst, err := LoadScript("test_input")
	require.NoError(t, err)

	acc := partTwo(inst)
	require.Equal(t, 8, acc)
}
