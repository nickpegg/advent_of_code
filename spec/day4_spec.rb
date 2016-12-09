require 'rspec'

require 'advent/day4/room'
require 'advent/day4/cipher'

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

describe Advent::Day4::Cipher do
  describe '#decrypt' do
    subject { described_class.decrypt('qzmt zixmtkozy ivhz', 343) }
    it { is_expected.to eq 'very encrypted name' }
  end
end
