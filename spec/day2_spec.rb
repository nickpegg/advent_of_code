require 'rspec'

require 'advent/day2/keypad'

describe Advent::Day2::Keypad do
  context 'with the example input' do
    let(:input) do
      "ULL
      RRDDD
      LURDL
      UUUUD"
    end

    subject { described_class.new(input) }

    it { expect(subject.solve).to eq [1, 9, 8, 5] }
  end
end
