package main

type Person struct {
	Answers map[rune]bool
}

type Questionaire struct {
	Questions []rune
	People    []Person
}

func NewQuestionaire(questions []rune) *Questionaire {
	q := &Questionaire{Questions: questions}
	return q
}

// AddPerson attaches a new person to the Questionaire which has the given answers
func (q *Questionaire) AddPerson(answers string) {
	// Add person and set answers to default
	p := Person{make(map[rune]bool)}
	for _, question := range q.Questions {
		p.Answers[question] = false
	}
	for _, ans := range answers {
		p.Answers[ans] = true
	}
	q.People = append(q.People, p)
}

// YesCount returns the number of answers which someone in the group said "yes" to
func (q *Questionaire) SomeoneYesCount() int {
	answers := make(map[rune]bool)
	for _, a := range q.Questions {
		answers[a] = false
	}

	// Reduce individual answers to whole-group answers
	for _, p := range q.People {
		for k, v := range p.Answers {
			if v {
				answers[k] = v
			}
		}
	}

	// count up "yes" answers
	c := 0
	for _, v := range answers {
		if v {
			c++
		}
	}
	return c
}

// EveryoneYesCount returns the count of questions where everyone in the group has said yes to
func (q *Questionaire) EveryoneYesCount() int {
	answers := make(map[rune]bool)
	// default true, and we'll mark false if anyone hasn't answered yes
	for _, q := range q.Questions {
		answers[q] = true
	}

	for _, question := range q.Questions {
		for _, p := range q.People {
			if !p.Answers[question] {
				answers[question] = false
				break
			}
		}
	}

	// count up questions everyone answered yes to
	c := 0
	for _, v := range answers {
		if v {
			c++
		}
	}
	return c
}
