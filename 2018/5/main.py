import concurrent.futures
import functools


def do_reduce(polymer):
    # type: (str) -> str
    new_polymer = ''

    i = 0
    j = 1
    while j < len(polymer):
        letter = polymer[i]
        next_letter = polymer[j]

        if abs(ord(letter) - ord(next_letter)) == 32:
            i += 2
            j += 2
        else:
            new_polymer += letter
            i += 1
            j += 1

    if j == len(polymer):
        new_polymer += polymer[-1]

    return new_polymer


def react(polymer):
    # type: (str) -> str
    last = polymer
    polymer = do_reduce(polymer)
    while last != polymer:
        last = polymer
        polymer = do_reduce(polymer)

    return polymer


def fast_react(polymer):
    # type: (str) -> str
    p = list(polymer)
    i = 0
    j = 1

    while j < len(p):
        if abs(ord(p[i]) - ord(p[j])) == 32:
            del p[j]
            del p[i]
            if i != 0:
                i -= 1
                j -= 1
        else:
            i += 1
            j += 1

    return ''.join(p)


def solution1(jawn):
    # type: (str) -> str
    result = fast_react(jawn)
    print(f"Solution 1: {len(result)}")
    return result


def remove_letter(letter, polymer):
    # type: (str, str) -> str
    """
    Remove ``letter`` from the polymer, both the upper and lower case version
    of it
    """
    new_polymer = ''
    bad_chars = {letter.upper(), letter.lower()}

    for l in polymer:
        if l not in bad_chars:
            new_polymer += l

    return new_polymer


def remove_react(polymer, letter):
    # type: (str, str) -> str
    """
    React a polymer after removing a letter
    """
    print(f"Removing {letter} and reacting...")
    return fast_react(remove_letter(letter, polymer))


def solution2(jawn):
    # type: (str) -> str
    # answer: 5698
    pool = concurrent.futures.ProcessPoolExecutor()
    a_to_z = map(chr, range(ord('a'), ord('z') + 1))

    results = []
    for letter in a_to_z:
        result = pool.submit(remove_react, jawn, letter)
        results.append(result)

    best = None
    for f in concurrent.futures.as_completed(results):
        reacted = f.result()
        print(f"Result length: {len(reacted)}")
        if best is None or len(best) > len(reacted):
            best = reacted

    pool.shutdown()

    if best is None:
        raise RuntimeError("Nothing was reacted? what.")

    print(f"The best is length {len(best)}")
    return best


def main():
    polymer = open('input.txt').read().strip()
    solution1(polymer)
    solution2(polymer)


if __name__ == '__main__':
    main()
