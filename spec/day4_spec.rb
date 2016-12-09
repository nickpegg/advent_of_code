require 'rspec'

require 'advent/day4/room'

describe Advent::Day4::Room do
  valid_roomspecs = %w(
    aaaaa-bbb-z-y-x-123[abxyz]
    a-b-c-d-e-f-g-h-987[abcde]
    not-a-real-room-404[oarel]
  )

  invalid_roomspecs = %w(
    totally-real-room-200[decoy]
  )

  describe '#valid?' do
    valid_roomspecs.each do |rs|
      context "when given #{rs}" do
        subject { described_class.new(rs).valid? }
        it { is_expected.to be_truthy }
      end
    end

    invalid_roomspecs.each do |rs|
      context "when given #{rs}" do
        subject { described_class.new(rs).valid? }
        it { is_expected.to be_falsey }
      end
    end
  end
end
