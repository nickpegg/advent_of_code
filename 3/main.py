import re
from pprint import pprint

from typing import List, NewType, Set, Tuple


class InvalidSpec(RuntimeError):
    pass


# 3-dimensional grid: columns, rows, then Claims
class Grid(object):
    def __init__(self):
        self.rows = [[]]    # type: List[List[List[int]]]
        self.width = 1
        self.height = 1

        self._overlapping_points = set()
        self._overlapping_claims = set()

    def add_claim(self, claim):
        # type: (Claim) -> None
        # First, ensure our grid is big enough
        max_column = claim.left + claim.width
        max_row = claim.top + claim.height
        self.width = max(self.width, max_column)
        self.height = max(self.height, max_row)

        for row in self.rows:
            if len(row) < self.width:
                extend_size = self.width - len(row)
                for i in range(extend_size):
                    row.append([])

        if len(self.rows) < self.height:
            extend_size = self.height - len(self.rows)
            for i in range(extend_size):
                blank_row = []  # type: List[list]
                for i in range(self.width):
                    blank_row.append([])
                self.rows.append(blank_row)

        # Now that the grid is big enough, add the claim ID to its positions
        for y in range(claim.top, claim.top + claim.height):
            for x in range(claim.left, claim.left + claim.width):
                square = self.rows[y][x]
                square.append(claim.id)
                if len(square) > 1:
                    self._overlapping_claims.update(square)
                    self._overlapping_points.add((x,y))

    @property
    def overlapping_points(self):
        # type: () -> List[Tuple[int, int]]
        return self._overlapping_points

    @property
    def overlapping_claim_ids(self):
        # type: () -> Set[int]
        return self._overlapping_claims

    @property
    def reduced_grid(self):
        # type () -> List[List[Union[int, str]]]
        blank_row = ['.'] * self.width
        reduced = [blank_row] * self.height
        for y in range(self.height):
            for x in range(self.width):
                ids = self.rows[y][x]
                if len(ids) == 1:
                    reduced[y][x] = ids[0]
                elif len(ids) > 1:
                    reduced[y][x] = 'X'

        return reduced


class Claim(object):
    def __init__(self, _id, left, top, width, height):
        # type: (int, int, int, int, int) -> None
        self.id = _id
        self.left = left
        self.top = top
        self.width = width
        self.height = height

    def __repr__(self):
        # type () -> str
        return "<Claim #{} @ {},{}: {}x{}>".format(
            self.id,
            self.left,
            self.top,
            self.width,
            self.height,
        )

    @classmethod
    def from_spec(cls, spec):
        # type: (str) -> Claim
        spec_re = r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)"
        match = re.match(spec_re, spec)

        if match:
            values = map(int, match.groups())
            return cls(*values)
        else:
            raise InvalidSpec


def solution():
    grid = Grid()
    claims = {}
    for spec in open('input.txt').readlines():
        spec = spec.strip()
        claim = Claim.from_spec(spec)
        grid.add_claim(claim)
        claims[claim.id] = claim

    claim_ids = set(claims.keys())

    print("Number of overlapping points: {}".format(len(grid.overlapping_points)))

    good_claims = claim_ids - grid.overlapping_claim_ids
    assert len(good_claims) == 1
    print("Non-overlapping claim: {}".format(claims[good_claims.pop()]))


if __name__ == '__main__':
    solution()
