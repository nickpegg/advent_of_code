require 'rspec'

require 'advent/day6/decoder'

describe Advent::Day6::Decoder do
  describe '#decode' do
    let(:example_input) do
      %w(
        eedadn
        drvtee
        eandsr
        raavrd
        atevrs
        tsrnev
        sdttsa
        rasrtv
        nssdts
        ntnada
        svetve
        tesnvt
        vntsnd
        vrdear
        dvrsen
        enarar
      ).join("\n")
    end

    context 'in most-freqent mode' do
      subject { described_class.new(example_input).decode }
      it { is_expected.to eq 'easter' }
    end

    context 'in lest-frequent mode' do
      subject { described_class.new(example_input, mode: :least).decode }
      it { is_expected.to eq 'advent' }
    end
  end
end
