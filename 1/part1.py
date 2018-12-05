#!/usr/bin/env python3

def main():
    acc = 0
    for freq_string in open('input.txt').readlines():
        freq = int(freq_string)
        acc += freq

    print(acc)



if __name__ == '__main__':
    main()
