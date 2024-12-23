package main

import (
	"fmt"
	"io/ioutil"
	"strings"
	//util "github.com/nickpegg/advent_of_code/2020/common"
)

var (
	QuestionaireQuestions = "abcdefghijklmnopqrstuvwxyz"
)

func main() {
	groups := loadFile("input")
	fmt.Println("Part 1:", partOne(groups))
	fmt.Println("Part 2:", partTwo(groups))
}

func loadFile(filename string) []string {
	data, err := ioutil.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	groups := strings.Split(string(data), "\n\n")
	return groups
}

func partOne(groups []string) int {
	sum := 0

	for _, group := range groups {
		q := NewQuestionaire([]rune(QuestionaireQuestions))
		for _, person := range strings.Split(group, "\n") {
			q.AddPerson(person)
		}
		sum += q.SomeoneYesCount()
	}

	return sum
}

func partTwo(groups []string) int {
	sum := 0

	for _, group := range groups {
		q := NewQuestionaire([]rune(QuestionaireQuestions))
		for _, person := range strings.Split(group, "\n") {
			if person == "" {
				continue
			}
			q.AddPerson(person)
		}
		count := q.EveryoneYesCount()
		sum += count
	}

	return sum
}
