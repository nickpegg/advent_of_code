package main

import (
	"github.com/stretchr/testify/require"
	"testing"
)

func TestQSomeoneYesCount(t *testing.T) {
	q := NewQuestionaire([]rune{'a', 'b', 'c', 'x', 'y', 'z'})
	q.AddPerson("abx")
	require.Equal(t, 3, q.SomeoneYesCount(), q)
}

func TestQEveryoneYesCount(t *testing.T) {
	q := NewQuestionaire([]rune{'a', 'b', 'c', 'x', 'y', 'z'})
	q.AddPerson("abxy")
	q.AddPerson("zxa")
	require.Equal(t, 2, q.EveryoneYesCount(), q)
}
