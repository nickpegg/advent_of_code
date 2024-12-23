from expects import *
from mamba import before, context, describe, it

from main import Coord, Nanobot, solution1, solution2


def get_specs(input_file):
    return [l.strip() for l in open(input_file)]


with describe(Nanobot):
    with describe('from_spec()'):
        with context('with a rando spec') as self:
            with before.each:
                self.bot = Nanobot.from_spec('pos=<5,-9,24>, r=5')

            with it('can parse the spec'):
                expect(self.bot.pos.x).to(equal(5))
                expect(self.bot.pos.y).to(equal(-9))
                expect(self.bot.pos.z).to(equal(24))
                expect(self.bot.r).to(equal(5))

        with context('with the given specs') as self:
            with before.each:
                self.specs = get_specs('test_input.txt')

            with it('can parse those'):
                expected = [
                    (Coord(0,0,0), 4),
                    (Coord(1,0,0), 1),
                    (Coord(4,0,0), 3),
                    (Coord(0,2,0), 1),
                    (Coord(0,5,0), 3),
                    (Coord(0,0,3), 1),
                    (Coord(1,1,1), 1),
                    (Coord(1,1,2), 1),
                    (Coord(1,3,1), 1),
                ]

                for spec, (c, r) in zip(self.specs, expected):
                    bot = Nanobot.from_spec(spec)
                    expect(bot.pos).to(equal(c))
                    expect(bot.r).to(equal(r))

    with describe('in_range_of()') as self:
        with before.each:
            self.bot = Nanobot(Coord(0,0,0), 4)

        with it('considers 1,1,1 in range'):
            expect(self.bot.in_range_of(Coord(1,1,1))).to(be_true)

        with it('considers 5,0,0 out of range'):
            expect(self.bot.in_range_of(Coord(5,0,0))).to(be_false)

        with it('considers 2,2,1 out of range'):
            expect(self.bot.in_range_of(Coord(2,2,1))).to(be_false)


with describe(solution1) as self:
    with it('gives the right answer for the test specs'):
        num = solution1('test_input.txt')
        expect(num).to(equal(7))


with describe(solution2) as self:
    with it('gives the right answer for the test specs'):
        distance = solution2('test_input2.txt')
        expect(distance).to(equal(36))
