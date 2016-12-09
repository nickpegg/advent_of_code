module Advent
  module Day2
    # Represents the keypad and finger position,
    # 1 is 0,0, finger starts on the 5
    class Keypad
      attr_reader :pad, :presses
      attr_accessor :x, :y

      def initialize(input)
        @pad = [
          [1, 2, 3],
          [4, 5, 6],
          [7, 8, 9]
        ]

        # Initial finger position on the 5
        @x = 1
        @y = 1

        # Array containing the sequence of presses
        @presses = []
        @operations = input.split
      end

      def solve
        @operations.each do |operation|
          operation.split('').each do |direction|
            move direction
          end

          press
        end

        @presses
      end

      def move(direction)
        case direction
        when 'U'
          @y -= 1 if @y > 0
        when 'L'
          @x -= 1 if @x > 0
        when 'R'
          @x += 1 if @x < 2
        when 'D'
          @y += 1 if @y < 2
        else
          puts "Invalid finger move direction: #{direction}"
        end
      end

      def press
        @presses << @pad[y][x]
      end
    end
  end
end
