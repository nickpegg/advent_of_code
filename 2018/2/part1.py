#!/usr/bin/env python3

from collections import Counter, defaultdict


def main():
    buckets = defaultdict(list)

    for line in open('input.txt').readlines():
        line = line.strip()

        c = Counter(line)
        for count in set(c.values()):
            if count != 1:
                buckets[count].append(line)

    checksum = 1
    for jawns in buckets.values():
        print(len(jawns))
        checksum *= len(jawns)

    print(len(buckets[2]) * len(buckets[3]))


if __name__ == '__main__':
    main()
