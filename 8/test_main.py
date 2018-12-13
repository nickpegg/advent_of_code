import pytest

from main import Node, parse_nodes, solution1, solution2


@pytest.fixture
def test_nodes():
    return parse_nodes('test_input.txt')


def test_parse_nodes(test_nodes):
    a = Node(metadata=[1, 1, 2])
    b = Node(metadata=[10, 11, 12])
    c = Node(metadata=[2])
    d = Node(metadata=[99])

    c.children = [d]
    a.children = [b, c]

    expected = a

    assert test_nodes == expected


def test_solution1(test_nodes):
    assert solution1(test_nodes) == 138


def test_solution2(test_nodes):
    assert solution2(test_nodes) == 66
