package main

import (
	"errors"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

type Passport struct {
	PassportId     string
	CountryId      int
	IssueYear      int
	ExpirationYear int

	BirthYear int
	Height    string
	EyeColor  string
	HairColor string
}

// NewPassport returns a Passport created from the given string record. This makes no guarantees
// about the validity of the passport, use the Errors() method for checking that.
//
// Example record:
//   hcl:#ae17e1 iyr:2013
//   eyr:2024
//   ecl:brn pid:760753108 byr:1931
//   hgt:179cm
func NewPassport(record string) (*Passport, error) {
	p := &Passport{}

	// Parse the record character by character
	fieldName := ""
	fieldValue := ""
	gotName := false
	for _, c := range []rune(record) {
		if c == ':' {
			gotName = true
			continue
		} else if c == '\n' || c == ' ' {
			// got a full field
			p.setField(fieldName, fieldValue)

			fieldName = ""
			fieldValue = ""
			gotName = false
			continue
		}

		if gotName {
			fieldValue += string(c)
		} else {
			fieldName += string(c)
		}
	}
	p.setField(fieldName, fieldValue)
	return p, nil
}

func (p *Passport) setField(fieldName string, fieldValue string) error {
	switch fieldName {
	case "byr":
		val, err := strconv.Atoi(fieldValue)
		if err != nil {
			return err
		}
		p.BirthYear = val
	case "iyr":
		val, err := strconv.Atoi(fieldValue)
		if err != nil {
			return err
		}
		p.IssueYear = val
	case "eyr":
		val, err := strconv.Atoi(fieldValue)
		if err != nil {
			return err
		}
		p.ExpirationYear = val
	case "hgt":
		p.Height = fieldValue
	case "ecl":
		p.EyeColor = fieldValue
	case "hcl":
		p.HairColor = fieldValue
	case "pid":
		p.PassportId = fieldValue
	case "cid":
		val, err := strconv.Atoi(fieldValue)
		if err != nil {
			return err
		}
		p.CountryId = val
	case "":
		break
	default:
		fmt.Println("Invalid field name:", fieldName)
	}

	return nil
}

// Errors returns an error if the Passport is not valid.
func (p *Passport) Errors() error {
	missingFields := make([]string, 0)

	if p.BirthYear == 0 {
		missingFields = append(missingFields, "byr")
	} else if p.IssueYear == 0 {
		missingFields = append(missingFields, "iyr")
	} else if p.ExpirationYear == 0 {
		missingFields = append(missingFields, "eyr")
	} else if p.Height == "" {
		missingFields = append(missingFields, "hgt")
	} else if p.EyeColor == "" {
		missingFields = append(missingFields, "ecl")
	} else if p.HairColor == "" {
		missingFields = append(missingFields, "hcl")
	} else if p.PassportId == "" {
		missingFields = append(missingFields, "pid")
	}

	if len(missingFields) > 0 {
		msg := fmt.Sprint("Missing fields: ", strings.Join(missingFields, ", "))
		return errors.New(msg)
	} else {
		return nil
	}
}

// DataErrors returns an error if the Passport's data values are not valid
func (p *Passport) DataErrors() error {
	if p.BirthYear < 1920 || p.BirthYear > 2002 {
		return fmt.Errorf("Invalid birth year: %v", p.BirthYear)
	} else if p.IssueYear < 2010 || p.IssueYear > 2020 {
		return fmt.Errorf("Invalid issue year: %v", p.IssueYear)
	} else if p.ExpirationYear < 2020 || p.ExpirationYear > 2030 {
		return fmt.Errorf("Invalid expiration year: %v", p.ExpirationYear)
	}

	// Check Height parameters, cm or in
	heightRE := regexp.MustCompile(`(\d+)(cm|in)`)
	matches := heightRE.FindStringSubmatch(p.Height)
	if len(matches) != 3 {
		return fmt.Errorf("Invalid height: %v", p.Height)
	}
	heightVal, err := strconv.Atoi(matches[1])
	if err != nil {
		return fmt.Errorf("Invalid height %v, %w", matches[1], err)
	}
	if matches[2] == "in" {
		if heightVal < 59 || heightVal > 76 {
			return fmt.Errorf("Height in inches out of bounds: %v", heightVal)
		}
	} else if matches[2] == "cm" {
		if heightVal < 150 || heightVal > 193 {
			return fmt.Errorf("Height in cm out of bounds: %v", heightVal)
		}
	} else {
		return fmt.Errorf("Invalid height unit: %v", matches[2])
	}

	// Check Hair color - HTML hex code
	if len(p.HairColor) != 7 {
		return fmt.Errorf("Invalid HairColor length: %v", p.HairColor)
	}
	if p.HairColor[0] != '#' {
		return fmt.Errorf("HairColor does not start with #: %v", p.HairColor)
	}
	_, err = strconv.ParseInt(p.HairColor[1:], 16, 64)
	if err != nil {
		return fmt.Errorf("Invalid HairColor %v: %e", p.HairColor, err)
	}

	// Check Eye color
	validEyeColors := map[string]bool{
		"amb": true,
		"blu": true,
		"brn": true,
		"gry": true,
		"grn": true,
		"hzl": true,
		"oth": true,
	}
	_, vec := validEyeColors[p.EyeColor]
	if !vec {
		return fmt.Errorf("Invalid eye color: %v", p.EyeColor)
	}

	// Check passport ID - 9 digit number counting leading 0s
	_, err = strconv.Atoi(p.PassportId)
	if err != nil {
		return err
	}
	if len(p.PassportId) != 9 {
		return errors.New(fmt.Sprintf("PID length != 9 - %v", p.PassportId))
	}

	return nil
}
