import pytest

import main
from main import Point


@pytest.fixture
def test_coords():
    return main.load_points('test_input.txt')


class TestGrid(object):
    def test_expand(self):
        grid = main.Grid()
        grid.expand(Point(5,6))

        for column in grid.grid:
            assert len(column) == 7
        assert len(grid.grid) == 6

    def test_add_point(self):
        grid = main.Grid()
        grid.add_point(Point(5,6), 'foo')

        assert grid.grid[5][6] == 'foo'

    def test_display(self, test_coords):
        g = main.Grid()
        for _id, p in test_coords.items():
            g.add_point(p, _id)

        actual = g.display()
        expected = open('test_display.txt').read()
        assert actual == expected
        print(actual)


def test_manhattan_distance():
    assert main.manhattan_distance(Point(0,0), Point(6,6)) == 12
    assert main.manhattan_distance(Point(0,0), Point(0,0)) == 0
    assert main.manhattan_distance(Point(0,0), Point(0,6)) == 6


def test_solution1(test_coords):
    assert main.solution1(test_coords) == 17


def test_solution2(test_coords):
    g = main.Grid(region_max_distance=32)
    assert main.solution2(test_coords, grid=g) == 16
