require 'rspec'

require 'advent/day5/password'

describe Advent::Day5::Password do
  describe '#generate' do
    subject { described_class.new('abc').generate }
    it { is_expected.to eq '18f47a30' }
  end
end
