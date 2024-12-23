package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestSeat(t *testing.T) {
	z := &Zone{5, 5, 7, 7}
	r, c, err := z.Seat()
	require.NoError(t, err)
	require.Equal(t, 5, r)
	require.Equal(t, 7, c)

	z.rowStart = 1
	_, _, err = z.Seat()
	require.Error(t, err)
}

func TestBisect(t *testing.T) {
	// path from the example in the thing
	type TestCase struct {
		rule rune
		zone *Zone
	}
	testCases := []TestCase{
		TestCase{'F', &Zone{0, 63, 0, 7}},
		TestCase{'B', &Zone{32, 63, 0, 7}},
		TestCase{'F', &Zone{32, 47, 0, 7}},
		TestCase{'B', &Zone{40, 47, 0, 7}},
		TestCase{'B', &Zone{44, 47, 0, 7}},
		TestCase{'F', &Zone{44, 45, 0, 7}},
		TestCase{'F', &Zone{44, 44, 0, 7}},
		TestCase{'R', &Zone{44, 44, 4, 7}},
		TestCase{'L', &Zone{44, 44, 4, 5}},
		TestCase{'R', &Zone{44, 44, 5, 5}},
	}

	zone := NewZone(128, 8)
	var err error
	var acc []rune
	for _, testCase := range testCases {
		acc = append(acc, testCase.rule)
		t.Log("Before:", zone)
		zone, err = zone.Bisect(testCase.rule)
		t.Log("After:", zone)
		require.NoError(t, err)
		require.Equal(t, testCase.zone, zone, string(acc))
	}
}

func TestBisectFull(t *testing.T) {
	var err error

	type testCase struct {
		in  string
		row int
		col int
	}
	testCases := []testCase{
		testCase{"BFFFBBFRRR", 70, 7},
		testCase{"FFFBBBFRRR", 14, 7},
		testCase{"BBFFBBFRLL", 102, 4},
	}

	for _, tc := range testCases {
		zone := NewZone(128, 8)
		for _, rule := range tc.in {
			zone, err = zone.Bisect(rule)
			require.NoError(t, err)
		}
		row, col, err := zone.Seat()
		require.NoError(t, err)
		require.Equal(t, tc.row, row)
		require.Equal(t, tc.col, col)
	}
}
