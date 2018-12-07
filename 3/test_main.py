import pytest
from pprint import pprint

from main import Claim, Grid, InvalidSpec


@pytest.fixture
def good_specs():
    return [
        '#1 @ 1,3: 4x4',
        '#2 @ 3,1: 4x4',
        '#3 @ 5,5: 2x2',
    ]


class TestClaim(object):
    def test_from_spec(self, good_specs):
        expected = (
            # id, left, top, width, height
            (1, 1, 3, 4, 4),
            (2, 3, 1, 4, 4),
            (3, 5, 5, 2, 2),
        )

        for spec, expect in zip(good_specs, expected):
            _id, left, top, width, height = expect
            claim = Claim.from_spec(spec)

            assert claim.id == _id
            assert claim.left == left
            assert claim.top == top
            assert claim.width == width
            assert claim.height == height

    def test_from_invalid_spec(self):
        bad_specs = (
            '#1@1,3:4x5',
            'farts',
        )

        for spec in bad_specs:
            with pytest.raises(InvalidSpec):
                Claim.from_spec(spec)


class TestGrid(object):
    @pytest.fixture
    def grid(self, good_specs):
        grid = Grid()
        for spec in good_specs:
            grid.add_claim(Claim.from_spec(spec))
        return grid

    def test_sized_properly(self, grid):
        """
        Assert that the grid is sized appropriately when specs are added,
        meaning no IndexError is raised
        """
        assert grid.width == 7
        assert grid.height == 7

        assert len(grid.rows) == 7
        for row in grid.rows:
            assert len(row) == 7

    def test_basic_case(self, grid):
        pprint(grid.reduced_grid)
        assert len(grid.overlapping_points) == 4

    def test_no_duplicate_ids(self, grid):
        """
        Grid positions don't have duplicate IDs in them.
        """
        for row in grid.rows:
            for point in row:
                assert len(point) == len(set(point))

    def test_no_reused_position_lists(self, grid):
        """
        Assert that all grid positions are unique lists and that I didn't screw
        anything up
        """
        ids = set()
        for row in grid.rows:
            for position in row:
                ids.add(id(position))

        assert len(ids) == grid.height * grid.width
