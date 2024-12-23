import pytest

from .util import Point, manhattan_distance


@pytest.mark.parametrize(
    "point1,point2,expected",
    [
        (Point(0, 0), Point(0, 0), 0),
        (Point(0, 1), Point(0, 0), 1),
        (Point(1, 1), Point(0, 0), 2),
        (Point(-5, -3), Point(0, 0), 8),
        (Point(-5, -3), Point(20, 5), 33),
    ],
)
def test_manhattan_distance(point1: Point, point2: Point, expected: int) -> None:
    assert manhattan_distance(point1, point2) == expected
