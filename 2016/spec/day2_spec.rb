require 'rspec'

require 'advent/day2/keypad'

describe Advent::Day2::Keypad do
  let(:input) do
    "ULL
     RRDDD
     LURDL
     UUUUD"
  end

  context 'with the example input' do
    subject { described_class.new(input) }

    it { expect(subject.solve).to eq [1, 9, 8, 5] }
  end

  context 'with the actual keypad and example input' do
    subject { described_class.new(input, pad_type: :actual) }

    it { expect(subject.solve).to eq [5, 'D', 'B', 3] }
  end
end
