package main

import (
	"fmt"
	"io/ioutil"
	"strings"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestNewPassport(t *testing.T) {
	data, err := ioutil.ReadFile("test_input")
	if err != nil {
		t.Fatal(err)
	}
	passports := strings.Split(string(data), "\n\n")

	p, err := NewPassport(passports[0])
	require.NoError(t, err)
	require.Equal(t, "gry", p.EyeColor, passports[0])
	require.Equal(t, "860033327", p.PassportId, passports[0])
	require.Equal(t, 2020, p.ExpirationYear, passports[0])
	require.Equal(t, 1937, p.BirthYear, passports[0])
	require.Equal(t, 2017, p.IssueYear, passports[0])
	require.Equal(t, 147, p.CountryId, passports[0])
	require.Equal(t, "#fffffd", p.HairColor, passports[0])
	require.Equal(t, "183cm", p.Height, passports[0])
}

func TestPassportErrors(t *testing.T) {
	data, err := ioutil.ReadFile("test_input")
	if err != nil {
		t.Fatal(err)
	}
	passports := strings.Split(string(data), "\n\n")

	type testCase struct {
		Data    string
		Valid   bool
		Message string
	}
	cases := []testCase{
		testCase{Data: passports[0], Valid: true},
		testCase{Data: passports[1], Valid: false, Message: "Missing fields: hgt"},
		testCase{Data: passports[2], Valid: true},
		testCase{Data: passports[3], Valid: false, Message: "Missing fields: byr"},
	}

	for _, tcase := range cases {
		passport, err := NewPassport(tcase.Data)
		require.NoError(t, err)

		err = passport.Errors()
		if tcase.Valid {
			require.NoError(t, err, tcase.Data)
		} else {
			require.Error(t, err)
			if tcase.Message != "" {
				require.Equal(t, fmt.Sprintf("%s", err), tcase.Message)
			}
		}

	}
}

func TestPassportDataErrors(t *testing.T) {
	passports, err := loadPassports("valid_passport_data")
	if err != nil {
		t.Fatal(err)
	}
	for _, p := range passports {
		require.NoError(t, p.DataErrors(), p)
	}

	passports, err = loadPassports("invalid_passport_data")
	if err != nil {
		t.Fatal(err)
	}
	for _, p := range passports {
		require.Error(t, p.DataErrors(), p)
	}
}
