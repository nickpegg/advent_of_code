#!/usr/bin/env python

from pathlib import Path


def part1():
    print("==== PART 1 ====")
    lines = Path.open("aoc2023/day1/input").read().splitlines()

    nums = []
    for line in lines:
        first_num = ""
        last_num = ""

        for char in line:
            try:
                num = int(char)
            except ValueError:
                continue

            if first_num == "":
                first_num = char
            last_num = char

        line_num = int(first_num + last_num)
        print(f"Got {line_num} from {line}")
        nums.append(line_num)

    print(sum(nums))


def part2():
    print("==== PART 2 ====")
    number_words = {
        "one": "1",
        "two": "2",
        "three": "3",
        "four": "4",
        "five": "5",
        "six": "6",
        "seven": "7",
        "eight": "8",
        "nine": "9",
        "zero": "0",
    }
    lines = Path.open("aoc2023/day1/input").read().splitlines()

    nums = []
    for line in lines:
        accum = ""
        first_num = ""
        last_num = ""

        for char in line:
            accum += char
            good_char = ""

            try:
                num = int(char)
                good_char = char
            except ValueError:
                pass

            if not good_char:
                for number_word, val in number_words.items():
                    if accum.endswith(number_word):
                        good_char = val
                        break

            if good_char:
                if first_num == "":
                    first_num = good_char
                last_num = good_char

        line_num = int(first_num + last_num)
        print(f"Got {line_num} from {line}")
        nums.append(line_num)

    print(sum(nums))


if __name__ == "__main__":
    # part1()
    part2()
