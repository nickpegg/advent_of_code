from typing import NamedTuple


Point = NamedTuple("Point", [("x", int), ("y", int)])


def manhattan_distance(p1: Point, p2: Point) -> int:
    return abs(p1.x - p2.x) + abs(p1.y - p2.y)
