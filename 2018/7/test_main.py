import pytest

import main


@pytest.fixture
def step_lines():
    return [l.strip() for l in open('test_input.txt').readlines()]


@pytest.fixture
def steps():
    return main.prepare_steps('test_input.txt')


def test_parse_line(step_lines):
    expected = [
        ('c', 'a'),
        ('c', 'f'),
        ('a', 'b'),
        ('a', 'd'),
        ('b', 'e'),
        ('d', 'e'),
        ('f', 'e'),
    ]

    for l, e in zip(step_lines, expected):
        assert main.parse_line(l) == e


def test_prepare_steps(steps):
    assert [s.letter for s in sorted(steps['e'].parents)] == ['b', 'd', 'f']


def test_solution1(steps):
    assert main.solution1(steps) == 'CABDFE'.lower()


def test_solution2(steps):
    t = main.solution2(steps, base_seconds=0, pool_size=2)
    assert t == 15


def test_time_for_task():
    assert main.time_for_task('a') == 61
    assert main.time_for_task('z') == 86
    assert main.time_for_task('a', base_seconds=0) == 1

