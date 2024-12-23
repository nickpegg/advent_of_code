import copy
from typing import Dict, Iterable, List, NamedTuple, Optional, Set
from pprint import pprint


Point = NamedTuple('Point', (('x', int), ('y', int)))


class Grid(object):
    def __init__(self, region_max_distance=10000):
        # Access like grid[x][y]
        self.grid = [[None]]    # type: List[List[Optional[int]]]
        self.width = 1
        self.height = 1
        self.points = {}    # type: Dict[Point, int]
        self.region_max_distance = region_max_distance

    @property
    def all_points(self):
        # type: () -> Iterable[Point]
        """
        All points in the grid, not just coordinates
        """
        for x in range(self.width):
            for y in range(self.height):
                yield Point(x, y)

    def add_point(self, point, value):
        # type: (Point, int) -> None
        self.expand(point)
        self.grid[point.x][point.y] = value
        self.points[point] = value

    def expand(self, point):
        # type: (Point) -> None
        """
        Expand the grid to include the coordinates ``point``
        """
        if self.width <= point.x or self.height <= point.y:
            new_width = max(self.width, point.x + 1)
            new_height = max(self.height, point.y + 1)

            for i in range(new_width):
                if i >= len(self.grid):
                    self.grid.append([])
                column = self.grid[i]

                while len(column) < new_height:
                    column.append(None)

            self.width = new_width
            self.height = new_height

    def closest(self, point):
        # type: (Point) -> Optional[int]
        """
        Return the closest named point to the given point on the grid

        Returns None if >1 point is the closest distance away
        """
        distances = {}   # type: Dict[int, int]
        shortest = self.height + self.width
        for p, _id in self.points.items():
            distance = manhattan_distance(point, p)
            distances[_id] = distance

            if distance < shortest:
                shortest = distance

        possible = [k for k, v in distances.items() if v == shortest]

        if len(possible) > 1:
            return None
        else:
            return possible[0]

    def display(self):
        # type: () -> str
        new_grid = copy.deepcopy(self.grid)
        grid_str = ''

        for y in range(self.height):
            for x in range(self.width):
                p = Point(x, y)
                closest_point = self.closest(p)
                if closest_point is None:
                    grid_str += '.'
                else:
                    grid_str += str(closest_point)
            grid_str += "\n"

        return grid_str

    def count_closest_points(self):
        # type () -> Dict[int, int]
        """
        Count up how many "wins" each ID'd point has (the "won" point on the
        grid has the ID'd point as the closest ID'd point to it).

        Returns dict of id -> wins
        """
        wins = {}

        # Any winner that shows up on an edge (and goes out into infinity)
        # cannot be a winner
        losers = set()

        for p in self.all_points:
            closest_point = self.closest(p)

            if closest_point is not None:
                wins.setdefault(closest_point, 0)
                if self.on_edge(p):
                    losers.add(closest_point)
                wins[closest_point] += 1

        # filter out points that can't be winners
        # TODO: fix this, this is too aggressive
        for loser in losers:
            del wins[loser]

        return wins

    def on_edge(self, point):
        # type: (Point) -> bool
        if point.x == 0 or point.y == 0:
            return True
        elif point.x == self.width - 1 or point.y == self.height - 1:
            return True
        else:
            return False

    def point_in_region(self, point):
        # type: (Point) -> bool
        """
        Returns true if the Point is in the region, meaning is it less than
        10000 away from every coordinate (named point)
        """
        total_distance = 0
        for coord in self.points:
            total_distance += manhattan_distance(point, coord)

        return total_distance < self.region_max_distance

    def points_in_region(self):
        # type: () -> Set[Point]
        """
        Return all points in the grid where they fall in the region
        """
        return set(filter(self.point_in_region, self.all_points))


def manhattan_distance(point1, point2):
    # type: (Point, Point) -> int
    d_x = abs(point1.x - point2.x)
    d_y = abs(point1.y - point2.y)
    return d_x + d_y


def load_points(filename):
    # type: (str) -> Dict[int, Point]
    points = {}
    _id = 1

    lines = open(filename).readlines()
    for line in lines:
        x, y = line.split(',')
        p = Point(int(x), int(y))
        points[_id] = p
        _id += 1

    return points


def solution1(coords):
    # type: (Dict[int, Point]) -> int
    # attempts:
    # - 6127 - too high
    # - 3750 - too low
    # - 4938 - too high (binsearch)
    # - 403
    grid = Grid()
    for _id, p in coords.items():
        grid.add_point(p, _id)

    # double-check max x,y of points
    max_x = 0
    max_y = 0
    for p in grid.points:
        if p.x > max_x:
            max_x = p.x
        if p.y > max_y:
            max_y = p.y
    assert max_x == grid.width - 1
    assert max_y == grid.height - 1


    wins = grid.count_closest_points()
    winner = 0

    for _id, count in wins.items():
        if count > winner:
            winner = count

    if winner == 0:
        raise RuntimeError("unable to find winner")

    return winner


def solution2(coords, grid=None):
    if grid is None:
        grid = Grid()

    for _id, p in coords.items():
        grid.add_point(p, _id)

    return len(grid.points_in_region())


def main():
    coords = load_points('input.txt')
    print(f"solution 1: {solution1(coords)}")
    print(f"solution 2: {solution2(coords)}")


if __name__ == '__main__':
    main()
