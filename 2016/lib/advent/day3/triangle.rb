module Advent
  module Day3
    # Triangle class to tell if a given triangle is valid
    class Triangle
      attr_reader :sides

      def initialize(sides)
        @sides = sides
      end

      def valid?
        valid_num_sides? && valid_lengths?
      end

      def valid_num_sides?
        @sides.length == 3
      end

      def valid_lengths?
        a, b, c = sides.sort
        a + b > c
      end
    end
  end
end
