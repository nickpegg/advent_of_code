require 'rspec'

require 'advent/day5/positioned_password'

describe Advent::Day5::PositionedPassword do
  describe '#generate' do
    subject { described_class.new('abc').generate }
    it { is_expected.to eq '05ace8e3' }
  end
end
