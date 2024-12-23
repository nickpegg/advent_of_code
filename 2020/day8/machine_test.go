package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestRunNoLoop(t *testing.T) {
	ins, err := LoadScript("test_input")
	require.NoError(t, err)

	machine := &Machine{Program: ins}
	err = machine.Run()
	require.Error(t, err)
	require.Equal(t, 5, machine.Accumulator)
}
