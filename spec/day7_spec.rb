require 'rspec'
require 'advent/day7/ip_address'

describe Advent::Day7::IPAddress do
  subject { described_class }

  describe '#has_abba?' do
    subject { described_class.new '' }

    it { expect(subject.has_abba?('foof')).to be true }
    it { expect(subject.has_abba?('aaaa')).to be false }
    it { expect(subject.has_abba?('lol')).to be false }
    it { expect(subject.has_abba?('ajsghxllxasd')).to be true }
  end

  describe '#supports_tls?' do
    it { expect(subject.new('abba[mnop]qrst').supports_tls?).to be true }
    it { expect(subject.new('abcd[bddb]xyyx').supports_tls?).to be false }
    it { expect(subject.new('aaaa[qwer]tyui').supports_tls?).to be false }
    it { expect(subject.new('ioxxoj[asdfgh]zxcvbn').supports_tls?).to be true }

    it { expect(subject.new('aaaylfoofads[lolwut]howdy[lolno]').supports_tls?).to be true }
    it { expect(subject.new('ukuozdurxxrvljkoi[eysjyckwyiyuopa]fconkkukvvmgnvyn[nwkqsifcwlzjurruho]ryslsdfmhgesmdf').supports_tls?).to be false }
  end
end

