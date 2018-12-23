from expects import expect, equal, be_empty, contain, be_below, be_above_or_equal
from mamba import before, context, describe, it

from main import Coord, Nanobot


def get_specs(input_file):
    return [l.strip() for l in open(input_file)]


with describe(Nanobot) as self:
    with describe('from_spec') as spec_context:
        with context('with a rando spec'):
            with before.each:
                self.bot = Nanobot.from_spec('pos=<5,-9,24>, r=5')

            with it('can parse the spec'):
                expect(self.bot.pos.x).to(equal(5))
                expect(self.bot.pos.y).to(equal(-9))
                expect(self.bot.pos.z).to(equal(24))
                expect(self.bot.r).to(equal(5))

        with context('with the given specs'):
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

