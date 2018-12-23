import csv
from typing import Dict, List

from expects import expect, equal, be_empty, contain, be_below, be_above_or_equal
from mamba import before, context, describe, it

from main import Bag, Circle, solution1


def load_cases():
    # type: () -> List[Dict[str, int]]
    cases = []

    for input_case in csv.DictReader(open('test_input.csv')):
        case = {}
        for k, v in input_case.items():
            case[k] = int(v)

        cases.append(case)

    return cases


with describe(Bag) as self:
    with before.each:
        self.bag = Bag(5)

    with it('has the highest marble'):
        expect(self.bag._marbles).to(contain(5))

    with it('always returns the lowest marble'):
        expect(self.bag.get()).to(equal(0))
        expect(self.bag.get()).to(equal(1))
        expect(self.bag.get()).to(equal(2))

    with it('adds large marbles to the end'):
        self.bag.add(200)
        expect(list(self.bag._marbles)).to(equal([0,1,2,3,4,5,200]))

    with it('adds small marbles to the beginning'):
        # Get the first two marbles
        self.bag.get()
        self.bag.get()

        # Re-add 0
        self.bag.add(0)

        expect(list(self.bag._marbles)).to(equal([0,2,3,4,5]))

    with it('adds marbles to the middle as necessary'):
        self.bag._marbles.remove(3)
        self.bag._marbles.remove(4)

        self.bag.add(4)
        expect(list(self.bag._marbles)).to(equal([0,1,2,4,5]))


with describe(Circle) as self:
    with before.each:
        self.circle = Circle()

    with it('can add a marble when it is empty'):
        expect(self.circle.marbles).to(be_empty)

        self.circle.add(5)
        expect(self.circle.marbles).to(equal([5]))

    with it('looks right with 5 marbles added'):
        expect(self.circle.marbles).to(be_empty)

        for m in range(5):
            self.circle.add(m)

        expect(self.circle.marbles).to(equal([0,4,2,1,3]))

    with it('handles adding the marble 23 correctly'):
        for m in range(5):
            self.circle.add(m)
        self.circle.add(23)

        expect(self.circle.marbles).to(equal([0,4,2,1]))
        expect(self.circle._current_idx).to(be_below(len(self.circle.marbles)))
        expect(self.circle._current_idx).to(be_above_or_equal(0))


with describe(solution1) as self:
    with before.all:
        self.cases = load_cases()

    with it('works with the basic test cases'):
        for case in self.cases:
            expect(
                solution1(case['players'], case['last'])
            ).to(equal(case['score']))
