#!/usr/bin/env python3

from collections import Counter, defaultdict


def main():
    buckets = defaultdict(list)
    ids = [l.strip() for l in open('input.txt').readlines()]

    while ids:
        i = ids.pop()
        for j in ids:
            if i == j:
                continue
            if len(i) != len(j):
                print('warning, mismatched id lengths on {} and {}'.format(i,j))

            matching = 0
            common = ''
            for ic, jc in zip(i, j):
                if ic == jc:
                    matching += 1
                    common += ic

            if matching == len(i) - 1:
                print("common: {}".format(common))


if __name__ == '__main__':
    main()
