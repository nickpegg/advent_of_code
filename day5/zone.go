package main

import (
  "fmt"
)

type Zone struct{
  rowStart int
  rowEnd int
  colStart int
  colEnd int
}

// NewZone returns a new Zone with the given rows and columns
func NewZone(rows int, columns int) (*Zone) {
  return &Zone{0, rows-1, 0, columns-1}
}

// Copy returns a copy of the given zone
func (z *Zone) Copy() *Zone {
  return &Zone{z.rowStart, z.rowEnd, z.colStart, z.colEnd}
}

// Bisect returns a new Zone bisected by the given rule, which must be one of F, B, L, or R, which
// correspond to Front, Back, Left, or Right.
func (z *Zone) Bisect(rule rune) (*Zone, error) {
  newZone := z.Copy()

  if rule == 'F' {
    newZone.rowEnd = (z.rowEnd - z.rowStart) / 2 + z.rowStart
  } else if rule == 'B' {
    newZone.rowStart = (z.rowEnd - z.rowStart + 1) / 2 + z.rowStart
  } else if rule == 'L' {
    newZone.colEnd = (z.colEnd - z.colStart) / 2 + z.colStart
  } else if rule == 'R' {
    newZone.colStart = (z.colEnd - z.colStart + 1) / 2 + z.colStart
  } else {
    return nil, fmt.Errorf("Invalid rule: %v", rule)
  }

  return newZone, nil
}

// Seat returns the seat row and column if the Zone consists of a single seat. Otherwise, and error
// is returned.
func (z *Zone) Seat() (int, int, error) {
  if z.rowStart != z.rowEnd || z.colStart != z.colEnd {
    return 0, 0, fmt.Errorf("Zone is not a single seat: %v", z)
  } else {
    return z.rowStart, z.colStart, nil
  }
}

// SeatID returns the seat ID of the given seat
func SeatID(r int, c int) int {
  return r * 8 + c
}
