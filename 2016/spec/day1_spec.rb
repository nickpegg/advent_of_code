require 'rspec'

require 'advent/day1/solution'

describe Advent::Day1::Solution do
  {
    'R2, L3' => 5,
    'R2, R2, R2' => 2,
    'R5, L5, R5, R3' => 12
  }.each do |input, distance|
    subject do
      x, y = described_class.new(input).solve.first
      x.abs + y.abs
    end

    it "solves #{input} correctly" do
      x, y = described_class.new(input).solve.first
      expect(x.abs + y.abs).to eq distance
    end
  end

  it 'finds the first double-visited node for "R8, R4, R4, R8"' do
    x, y = described_class.new('R8, R4, R4, R8').solve.last
    expect(x.abs + y.abs).to eq 4
  end
end
