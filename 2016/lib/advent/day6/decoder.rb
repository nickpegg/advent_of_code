require 'pp'

module Advent
  module Day6
    # Decoder for repetition coding correction
    # The given `mode` determines whether we use the most common letters in
    # the given input, or the least common letters
    class Decoder
      def initialize(input, mode: :most)
        @debug = !ENV['DEBUG'].nil?
        @mode = mode
        @input = input.split
      end

      def decode
        letter_frequencies.map do |freq|
          sorted_frequencies = freq.sort_by { |k, v| v }

          if @mode == :most
            sorted_frequencies.last.first
          else
            sorted_frequencies.first.first
          end
        end.join
      end

      # Given the @input, return an Array of Hashes where the array index is
      # the position in the words and the hash has keys that are letters and
      # values that are the number of times a particular letter has been seen
      # in the list of words in the input
      def letter_frequencies
        freqs = []

        @input.each do |line|
          line.strip.split('').each_with_index do |letter, i|
            freqs[i] ||= Hash.new 0
            freqs[i][letter] += 1
          end
        end

        pp freqs if @debug
        freqs
      end
    end
  end
end
