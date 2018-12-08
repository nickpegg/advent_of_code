import pytest

from main import load_events, load_guards, solution1, solution2


@pytest.fixture
def guards():
    return load_guards(load_events('test_input.txt'))


def test_solution1(guards):
    """
    Basic test that soultion1 returns what we expect for the test input
    """
    guard, minute = solution1(guards)
    assert guard.id == 10
    assert minute == 24


def test_solution2(guards):
    guard, minute = solution2(guards)
    assert guard.id == 99
    assert minute == 45
