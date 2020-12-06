package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestPartOne(t *testing.T) {
	passports, err := loadPassports("test_input")
	require.NoError(t, err)
	require.Equal(t, 2, partOne(passports))
}
