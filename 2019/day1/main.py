def needed_fuel(mass: int) -> int:
    """
    Return how much fuel is needed for the given mass
    """
    return int(float(mass) / 3) - 2


def needed_fuel_with_fuel(mass: int) -> int:
    """
    Return how much fuel is needed for the given mass, and the fuel needed for that
    fuel, etc.
    """
    needed = int(float(mass) / 3) - 2
    if needed > 0:
        # Need fuel to carry this mass, figure out the fuel to carry THAT fuel
        needed += needed_fuel_with_fuel(needed)
        return needed
    else:
        return 0


def main() -> None:
    fuel = 0
    with open("input.txt") as f:
        for line in f.readlines():
            fuel += needed_fuel(int(line.strip()))

    print(f"Part 1 needed fuel: {fuel}")

    # part 2
    fuel = 0
    with open("input.txt") as f:
        for line in f.readlines():
            fuel += needed_fuel_with_fuel(int(line.strip()))

    print(f"Part 2 needed fuel: {fuel}")


if __name__ == "__main__":
    main()
