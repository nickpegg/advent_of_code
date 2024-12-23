#!/usr/bin/env python3

def main():
    acc = 0
    known = set()
    found = False

    while not found:
        for freq_string in open('input.txt').readlines():
            freq = int(freq_string)
            acc += freq

            if not found and acc in known:
                print("First hit twice is: {}".format(acc))
                found = True
                break
            else:
                known.add(acc)

    print(acc)



if __name__ == '__main__':
    main()
