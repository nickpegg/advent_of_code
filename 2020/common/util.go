package common

import (
	"io/ioutil"
	"strings"
)

// ReadLines returns all of the lines from the given filename
func ReadLines(filename string) ([]string, error) {
  data, err := ioutil.ReadFile(filename)
  if err != nil {
    return []string{}, err
  }

  lines := strings.Split(string(data), "\n")
  // Filter out blank lines
  goodLines := make([]string, 0)
  for _, line := range(lines) {
    if line != "" {
      goodLines = append(goodLines, strings.TrimSpace(line))
    }
  }
  return goodLines, nil
}
