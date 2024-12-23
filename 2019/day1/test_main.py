import pytest

from .main import needed_fuel, needed_fuel_with_fuel


@pytest.mark.parametrize(
    "mass,expected", [(12, 2), (14, 2), (1969, 654), (100756, 33583)]
)
def test_needed_fuel(mass: int, expected: int) -> None:
    assert needed_fuel(mass) == expected


@pytest.mark.parametrize("mass,expected", [(1969, 966), (100756, 50346)])
def test_needed_fuel_with_fuel(mass: int, expected: int) -> None:
    assert needed_fuel_with_fuel(mass) == expected
