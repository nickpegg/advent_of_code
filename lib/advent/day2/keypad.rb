module Advent
  module Day2
    # Represents the keypad and finger position,
    # 1 is 0,0, finger starts on the 5
    class Keypad
      attr_reader :pad, :presses
      attr_accessor :x, :y

      def initialize(input, pad_type: :example)
        @debug = !ENV['DEBUG'].nil?
        @pad = make_pad(pad_type)

        # Initial finger position on the 5
        @x, @y = starting_position

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
        new_x, new_y = new_spot(direction)

        if fetch_key(new_x, new_y)
          @x = new_x
          @y = new_y
          puts "Moved finger to (#{@x},#{@y}) #{fetch_key(@x, @y)}" if @debug
        else
          # Disable this cop since it would cause a readability issue
          # rubocop:disable Style/IfInsideElse
          puts "Can't move finger to (#{new_x}, #{new_y}), would be off-pad" if @debug
          # rubocop:enable Style/IfInsideElse
        end
      end

      def new_spot(direction)
        new_x = @x
        new_y = @y

        case direction
        when 'U'
          new_y = @y - 1
        when 'L'
          new_x = @x - 1
        when 'R'
          new_x = @x + 1
        when 'D'
          new_y = @y + 1
        else
          puts "Invalid finger move direction: #{direction}"
        end

        [new_x, new_y]
      end

      def fetch_key(x, y)
        return nil if @pad[y].nil?
        return nil if y < 0 || y >= @pad.length
        return nil if x < 0 || x >= @pad[y].length

        @pad[x][y]
      end

      def press
        @presses << @pad[y][x]
      end

      # Returns the pad to use, either the 'example' one given on the keypad
      # document, or the actual one on the bathroom door
      def make_pad(mode = :example)
        case mode
        when :example
          example_pad
        when :actual
          actual_pad
        end
      end

      def example_pad
        [
          [1, 2, 3],
          [4, 5, 6],
          [7, 8, 9]
        ]
      end

      def actual_pad
        [
          [nil, nil, 1, nil, nil],
          [nil, 2, 3, 4, nil],
          [5, 6, 7, 8, 9],
          [nil, 'A', 'B', 'C', nil],
          [nil, nil, 'D', nil, nil]
        ]
      end

      # Find the starting position. We _always_ start on the 5 key
      def starting_position
        (0..@pad.length).each do |y|
          (0..@pad[y].length).each do |x|
            return x, y if @pad[y][x] == 5
          end
        end

        nil
      end
    end
  end
end
