package main

import (
	"testing"

	util "github.com/nickpegg/advent_of_code/2020/common"
	"github.com/stretchr/testify/require"
)

func TestNewBag(t *testing.T) {
	b, err := NewBag("dark orange bags contain 3 bright white bags, 4 muted yellow bags.")
	require.NoError(t, err)
	require.Equal(t, "dark orange", b.Color)

	require.Equal(t, "bright white", b.Rules[0].Color)
	require.Equal(t, 3, b.Rules[0].Qty)

	require.Equal(t, "muted yellow", b.Rules[1].Color)
	require.Equal(t, 4, b.Rules[1].Qty)

	b, err = NewBag("dotted black bags contain no other bags.")
	require.NoError(t, err)
	require.Equal(t, "dotted black", b.Color)
	require.Empty(t, b.Rules)
}

func TestBagGraphContains(t *testing.T) {
	lines, err := util.ReadLines("test_input")
	if err != nil {
		t.Fatal(err)
	}
	bm, err := NewBagMap(lines)
	require.NoError(t, err)

	bg, err := NewBagGraph(bm)
	require.NoError(t, err)

	colors := bg.Contains("shiny gold")
	require.Len(t, colors, 4)
	require.Contains(t, colors, "bright white")
	require.Contains(t, colors, "muted yellow")
	require.Contains(t, colors, "dark orange")
	require.Contains(t, colors, "light red")
}

func TestTotalBags(t *testing.T) {
	lines, err := util.ReadLines("test_input")
	require.NoError(t, err)
	bm, err := NewBagMap(lines)
	require.NoError(t, err)
	require.Equal(t, 32, bm.TotalBags("shiny gold")-1, bm)

	lines, err = util.ReadLines("test_input2")
	require.NoError(t, err)
	bm, err = NewBagMap(lines)
	require.NoError(t, err)
	require.Equal(t, 126, bm.TotalBags("shiny gold")-1, bm)
}
