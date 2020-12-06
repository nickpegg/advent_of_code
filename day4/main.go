package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	passports, err := loadPassports("input")
	if err != nil {
		panic(err)
	}

	fmt.Println("Part 1:", partOne(passports))
	fmt.Println("Part 2:", partTwo(passports))
}

func loadPassports(filename string) ([]*Passport, error) {
	data, err := ioutil.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	passport_data := strings.Split(string(data), "\n\n")

	passports := make([]*Passport, 0)
	for _, pdata := range passport_data {
		passport, err := NewPassport(pdata)
		if err != nil {
			return nil, err
		}
		passports = append(passports, passport)
	}

	return passports, nil
}

func partOne(passports []*Passport) int {
	validCount := 0
	for _, p := range passports {
		if p.Errors() == nil {
			validCount++
		}
	}
	return validCount
}

func partTwo(passports []*Passport) int {
	validCount := 0

	for _, p := range passports {
		if p.Errors() == nil && p.DataErrors() == nil {
			validCount++
		}
	}

	return validCount
}
