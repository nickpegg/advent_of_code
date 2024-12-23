import math
import re
from collections import defaultdict
from concurrent.futures import ProcessPoolExecutor, as_completed
from pprint import pprint

from typing import Any, Dict, Iterable, List, NamedTuple, Set

from progress.bar import IncrementalBar
from progress.spinner import Spinner


class Coord(object):
    def __init__(self, x=0, y=0, z=0):
        # type: (int, int, int) -> None
        self.x = x
        self.y = y
        self.z = z

    def __eq__(self, other):
        # type: (Any) -> bool
        assert isinstance(other, Coord)
        return self.x == other.x and self.y == other.y and self.z == other.z

    def __hash__(self):
        # type: () -> int
        return hash((self.x, self.y, self.z))

    def __repr__(self):
        # type: () -> str
        return f"<Coord: ({self.x}, {self.y}, {self.z})>"


class Nanobot(object):
    def __init__(self, pos, radius=0):
        # type: (Coord, int) -> None
        self.pos = pos
        self.r = radius

    def __hash__(self):
        # type: () -> int
        return hash((self.pos, self.r))

    def __repr__(self):
        # type: () -> str
        return f"<Nanobot pos=<{self.pos.x}, {self.pos.y}, {self.pos.z}>, r={self.r}>"

    @classmethod
    def from_spec(cls, spec):
        # type: (str) -> Nanobot
        expr = r'pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)'
        match = re.match(expr, spec)
        if match:
            x, y, z, r = map(int, match.groups())
            c = Coord(x, y, z)

            return cls(c, r)
        else:
            raise RuntimeError(f'Unable to parse Nanobot from spec {spec}')

    def in_range_of(self, other):
        # type: (Coord) -> bool
        """
        Returns True if the given Coord is in range of this Nanobot
        """
        return distance(self.pos, other) <= self.r

    def overlaps_with(self, other):
        # type: (Nanobot) -> bool
        """
        Returns True if the two bots' range-spheres overlap
        """
        d = distance(self.pos, other.pos)
        return d <= self.r + other.r


def distance(a, b):
    # type: (Coord, Coord) -> int
    dist = 0
    dist += abs(a.x - b.x)
    dist += abs(a.y - b.y)
    dist += abs(a.z - b.z)
    return dist


def solution1(input_file):
    # type: (str) -> int
    specs = (l.strip() for l in open(input_file))
    bots = [Nanobot.from_spec(s) for s in specs]

    largest = None
    for bot in bots:
        if largest is None or largest.r < bot.r:
            largest = bot

    if largest is None:
        raise RuntimeError('what. No bots or something?')

    bots_in_range = []
    for bot in bots:
        if largest.in_range_of(bot.pos):
            bots_in_range.append(bot)

    return len(bots_in_range)


def solution2(input_file):
    # type: (str) -> int
    best_distance = None
    specs = (l.strip() for l in open(input_file))
    bots = [Nanobot.from_spec(s) for s in specs]

    # Sort bots by radius descending
    bots = sorted(bots, key=lambda b: b.r, reverse=True)

    # Gather groups of bots that overlap
    overlapping = {}    # type: Dict[Nanobot, List[Nanobot]]
    for a in bots:
        overlapping[a] = list()
        for b in bots:
            if a == b:
                continue
            if a.overlaps_with(b):
                overlapping[a].append(b)

    # Find largest group of overlapping spheres
    groups = list()     # type: List[Set[Nanobot]]
    for k, v in overlapping.items():
        group = set(v)
        group.add(k)
        if group not in groups:
            groups.append(group)
    groups = sorted(groups, key=len, reverse=True)
    largest_length = len(groups[0])
    large_groups = [g for g in groups if len(g) == largest_length]

    possible_points = set()     # type: Set[Coord]
    possible_bots = list()
    for group in large_groups:
        possible_points |= overlapping_points(group)
        if len(possible_points) > 0:
            possible_bots = group
            break

    print(f'possible points: {possible_points}')
    # Sort possible points by how many bots are in range
    point_scores = {}   # type: Dict[Coord, int]
    for p in possible_points:
        point_scores.setdefault(p, 0)
        for b in possible_bots:
            point_scores[p] += 1

    best_points = sorted(point_scores.items(), key=lambda x: x[1], reverse=True)
    best_score = best_points[0][1]
    possible_points = {k for k, v in best_points if v == best_score}

    for p in possible_points:
        d = distance(p, Coord(0,0,0))
        if best_distance is None or d < best_distance:
            best_distance = d

    if best_distance is None:
        raise RuntimeError("got 0 possible points")

    return best_distance


def overlapping_points(group):
    # type: (Set[Nanobot]) -> Set[Coord]
    """
    Return all the points that exist in the intersection of all Nanobots
    """
    if len(group) < 2:
        return set()

    points = set()

    # Sort by size descending so we can pop the smallest off the list
    sorted_group = sorted(group, key=lambda b: b.r, reverse=True).copy()

    a = sorted_group.pop()
    b = sorted_group.pop()
    bounding = bounding_sphere_of_intersection(a, b)
    while bounding.r != 0 and len(sorted_group) > 0:
        next_sphere = sorted_group.pop()
        bounding = bounding_sphere_of_intersection(bounding, next_sphere)

    print(f'bounding: {bounding}')
    # Build a bounding cube around the bounding sphere as possible points
    c = bounding.pos
    r = bounding.r
    for x in range(c.x - r, c.x + r + 1):
        for y in range(c.y - r, c.y + r + 1):
            for z in range(c.z - r, c.z + r + 1):
                points.add(Coord(x, y, z))

    return points


def bounding_sphere_of_intersection(a, b):
    # type: (Nanobot, Nanobot) -> Nanobot

    # Ensure `a` is the larger sphere
    if b.r > a.r:
        a, b = b, a

    # If `b` is entirely within `a`, just return `b`
    if abs(a.r - b.r) >= distance(a.pos, b.pos):
        return b

    # Get the circle of the intersection, by its raidus and midpoint
    # Source of these equations:
    # http://www.ambrsoft.com/TrigoCalc/Sphere/TwoSpheres/Intersection.htm
    x1 = a.pos.x
    y1 = a.pos.y
    z1 = a.pos.z
    x2 = b.pos.x
    y2 = b.pos.y
    z2 = b.pos.z

    A = 2 * (x2 - x1)
    B = 2 * (y2 - y1)
    C = 2 * (z2 - z1)
    D = x1**2 - x2**2 + y1**2 - y2**2 + z1**2 - z2**2 - a.r**2 + b.r**2

    t = (x1 * A + y1 * B + z1 * C + D) / (A*(x1-x2) + B*(y1-y2) + C*(z1-z2))

    # get the center point of the circle formed by the intersection of a and b
    # We'll use this to make a bounding sphere, which contains this circle, and
    # return that
    x = x1 + t * (x2 - x1)
    y = y1 + t * (y2 - y1)
    z = z1 + t * (z2 - z1)

    # Find the radius
    r1 = a.r
    r2 = b.r
    d = distance(a.pos, b.pos)

    try:
        r = math.sqrt(4 * r1**2 * d**2 - (r1**2 + d**2 - r2**2)**2) / (2 * d)
    except ValueError:
        r = 1

    # Add 1 to the radius to account for error in x, y, and z
    r += 1
    return Nanobot(Coord(round(x), round(y), round(z)), math.ceil(r))


def overlapping_points_in_spheres(a, b):
    # type: (Nanobot, Nanobot) -> Set[Coord]
    """
    Return the set of all points that could be in the intersection of spheres
    ``a`` and ``b``
    """


    return set()


def bots_in_range_multi(coords, possible_bots):
    # type: (List[Coord], List[Nanobot]) -> Dict[Coord, List[Nanobot]]
    results = {}
    for c in coords:
        results.update(bots_in_range(c, possible_bots))
    return results


def bots_in_range(coord, possible_bots):
    # type: (Coord, List[Nanobot]) -> Dict[Coord, List[Nanobot]]
    in_range = []

    for bot in possible_bots:
        if bot.in_range_of(coord):
            in_range.append(bot)

    return {coord: in_range}


def main():
    # type: () -> None
    num = solution1('input.txt')
    print(f"First solution: {num}")

    # Bad answers:
    #   62436713 (too low)
    #   88437692 (too high)
    dist = solution2('input.txt')
    print(f"Second solution: {dist}")


if __name__ == '__main__':
    main()
