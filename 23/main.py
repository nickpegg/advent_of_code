import re

from typing import NamedTuple


class Coord(NamedTuple):
    x: int
    y: int
    z: int


class Nanobot(object):
    def __init__(self, pos, radius=0):
        # type: (Coord, int) -> None
        self.pos = pos
        self.r = radius

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
