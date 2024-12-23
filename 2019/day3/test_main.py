from day3.main import walk, get_crosses
from common.util import Point


def test_walk() -> None:
    points = walk("R8,U5,L5,D3".split(","))
    points2 = walk("U7,R6,D4,L4".split(","))
    crosses = get_crosses([points, points2])
    assert crosses == {Point(3, 3), Point(6, 5)}
