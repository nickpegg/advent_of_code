package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

type BagMap map[string]*Bag

// BagGraph is a directed graph represented as nested maps, where the first key is the inner bag
// color, and the second key is the outer bag color.
type BagGraph map[string]map[string]bool

type BagRule struct {
	Qty   int
	Color string
}

type Bag struct {
	Color string
	Rules []BagRule
}

// NewBagMap returns a BagMap created from the given list of rule lines
func NewBagMap(ruleLines []string) (BagMap, error) {
	bm := make(BagMap)

	for _, line := range ruleLines {
		bag, err := NewBag(line)
		if err != nil {
			return bm, err
		}
		bm[bag.Color] = bag
	}
	return bm, nil
}

// TotalBags returns the total number of bags which must be contained in the given bag color
func (bm BagMap) TotalBags(color string) int {
	bag, ex := bm[color]
	if !ex {
		return 0
	}

	count := 1
	for _, rule := range bag.Rules {
		count += rule.Qty * bm.TotalBags(rule.Color)
	}
	return count
}

func NewBagGraph(bm BagMap) (BagGraph, error) {
	bg := make(BagGraph)

	for _, bag := range bm {
		outerColor := bag.Color
		for _, rule := range bag.Rules {
			innerColor := rule.Color
			_, exists := bg[innerColor]
			if !exists {
				bg[innerColor] = make(map[string]bool)
			}
			bg[innerColor][outerColor] = true
		}
	}

	return bg, nil
}

// Inverse returns an inverted version of the BagGraph, where bg[x][y] -> bg[y][x]
func (bg BagGraph) Inverse() BagGraph {
	newGraph := make(BagGraph)
	for ko, _ := range bg {
		for ki, _ := range bg[ko] {
			if _, ex := newGraph[ki]; !ex {
				newGraph[ki] = make(map[string]bool)
			}
			newGraph[ki][ko] = bg[ko][ki]
		}
	}
	return newGraph
}

// Contains returns all of the colors which can recursively contain the given color. This BagGraph
// must be a graph of containees to containers.
func (bg BagGraph) Contains(color string) []string {
	seenColors := make(map[string]bool)
	todo := []string{color}

	for len(todo) > 0 {
		nextColor := todo[0]
		todo = todo[1:]
		if _, ex := bg[nextColor]; !ex {
			continue
		}
		for k, _ := range bg[nextColor] {
			if _, ex := seenColors[k]; ex {
				continue
			}
			seenColors[k] = true
			todo = append(todo, k)
		}

	}

	// Convert seenColors to a slice
	containers := make([]string, 0)
	for k, _ := range seenColors {
		if k != color {
			containers = append(containers, k)
		}
	}
	return containers
}

// NewBag parses a bag rule line and returns a new Bag object
func NewBag(ruleLine string) (*Bag, error) {
	bag := &Bag{}
	bag.Rules = make([]BagRule, 0)

	parts := strings.Split(ruleLine, " bags contain ")
	if len(parts) != 2 {
		return nil, fmt.Errorf("Invalid rule line: %v", ruleLine)
	}
	bag.Color = parts[0]

	if strings.Contains(parts[1], "no other bags") {
		return bag, nil
	}

	rules := strings.Split(parts[1], ",")
	re := regexp.MustCompile(`(\d+) (\w+\s+\w+) bags?`)
	for _, rule := range rules {
		matches := re.FindStringSubmatch(rule)
		if len(matches) != 3 {
			return nil, fmt.Errorf("Invalid rule '%v' in '%v'", rule, ruleLine)
		}
		qty, err := strconv.Atoi(matches[1])
		if err != nil {
			return nil, fmt.Errorf("Invalid integer in '%v' in '%v'", rule, ruleLine)
		}

		br := BagRule{Qty: qty, Color: matches[2]}
		bag.Rules = append(bag.Rules, br)
	}

	return bag, nil
}
