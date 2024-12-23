require 'rspec'

require 'advent/day3/triangle'

describe Advent::Day3::Triangle do
  describe '#valid?' do
    [
      [2, 2, 3],
      [5, 5, 5]
    ].each do |lengths|
      it "thinks #{lengths.map(&:to_s).join(',')} is valid" do
        triangle = described_class.new(lengths)
        expect(triangle.valid?).to be_truthy
      end
    end

    [
      [5, 10, 25],
      [-1, 3, 3],
      [-5, 5, 5],
      [1, 2, 3, 4]
    ].each do |lengths|
      it "thinks #{lengths.map(&:to_s).join(',')} is not valid" do
        triangle = described_class.new(lengths)
        expect(triangle.valid?).to be_falsey
      end
    end
  end
end
