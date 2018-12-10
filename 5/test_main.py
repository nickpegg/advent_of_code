import pytest
import time

from main import do_reduce, react, solution1, solution2, fast_react


@pytest.fixture
def polymer():
    return open('test_input.txt').read().strip()


def test_do_reduce(polymer):
    expected = 'dabAaCBAcaDA'
    reduced = do_reduce(polymer)

    assert reduced == expected


def test_react(polymer):
    expected = 'dabCBAcaDA'
    reacted = react(polymer)

    assert reacted == expected


def test_solution2(polymer):
    assert len(solution2(polymer)) == 4


def test_fast_react(polymer):
    original = react(polymer)
    new = fast_react(polymer)

    assert original == new
