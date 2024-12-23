from common.util import Point, manhattan_distance

from typing import List, Sequence, Set


def walk(instructions: Sequence[str]) -> Set[Point]:
    """
    Given a list of instructions, walk and return all the points touched. Instructions
    should take the form of direction on the 2D plane and how many spaces to go in that
    direction, e.g. "D8" goes 8 spaces down. Direction is relative to the plane.
    """
    pos = Point(0, 0)
    points = set()
    for instruction in instructions:
        if not instruction.strip():
            continue
        direction = instruction[0]
        spaces = int(instruction[1:])
        print(instruction)

        if direction == "U":
            for d in range(spaces + 1):
                p = Point(pos.x, pos.y + d)
                print(p)
                points.add(p)
            pos = Point(pos.x, pos.y + spaces)
        elif direction == "D":
            for d in range(spaces + 1):
                p = Point(pos.x, pos.y - d)
                print(p)
                points.add(p)
            pos = Point(pos.x, pos.y - spaces)
        elif direction == "L":
            for d in range(spaces + 1):
                p = Point(pos.x - d, pos.y)
                print(p)
                points.add(p)
            pos = Point(pos.x - spaces, pos.y)
        elif direction == "R":
            for d in range(spaces + 1):
                p = Point(pos.x + d, pos.y)
                print(p)
                points.add(p)
            pos = Point(pos.x + spaces, pos.y)

    return points


def get_crosses(wires: Sequence[Set[Point]]) -> Set[Point]:
    """
    Get all points where the wires (sets of Points) cross. The origin (0,0) doesn't count
    """
    if len(wires) < 2:
        return set()

    crosses = set()  # type: Set[Point]

    # WRONG. Need to pair up each wire, find the instersection between the pairs, then
    # OR those intersections together
    for i, wire1 in enumerate(wires):
        for j, wire2 in enumerate(wires):
            if i == j:
                continue
            crosses |= wire1 & wire2

    crosses.remove(Point(0, 0))
    return crosses


def main() -> None:
    wires = []  # List[Set[Point]]
    with open("day3/input.txt") as f:
        for line in f.readlines():
            instructions = line.split(",")
            wires.append(walk(instructions))

    crosses = wires[0]
    for wire in wires[1:]:
        crosses &= wire
    print(f"wires cross at: {crosses}")


if __name__ == "__main__":
    main()
