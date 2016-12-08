#!/usr/bin/env ruby

module Advent
  module Day1
    # Solves the Day 1 challenge
    class Solution
      def initialize(input, debug: false)
        @debug_mode = debug

        @coords = [0, 0]
        @direction = 0
        @visits = Hash.new 0
        @first_double_visit = nil

        @commands = input.split(', ')
      end

      def solve
        @commands.each do |cmd|
          puts "Dealing with #{cmd}" if @debug_mode
          dir, distance = cmd.match(/(\w)(\d+)/)[1..-1]
          distance = distance.to_i

          turn dir
          move distance
        end

        [@coords, @first_double_visit]
      end

      def print_solution
        final_coords, double_visit = solve

        puts "First double visit is #{double_visit[0].abs + double_visit[1].abs} blocks away"
        puts "Final distance: #{final_coords[0].abs + final_coords[1].abs}"
      end

      def turn(dir)
        if dir == 'R'
          @direction += 90
        elsif dir == 'L'
          @direction -= 90
        else
          puts "Incorrect turn direction: #{dir}"
        end

        clamp_direction

        puts "Turned #{dir}, now facing #{@direction}" if @debug_mode
      end

      # Clamp direction to 0..359
      def clamp_direction
        if @direction < 0
          @direction += 360
        elsif @direction >= 360
          @direction -= 360
        end
      end

      def move(distance)
        case @direction / 90
        when 0
          take_steps(distance, coord: :y)
        when 1
          take_steps(distance)
        when 2
          take_steps(distance, coord: :y, direction: -1)
        when 3
          take_steps(distance, direction: -1)
        else
          puts "Invalid direction amount: #{@direction}"
        end
      end

      # Take the number of steps in the given coordinate. Direction should be 1
      # for stepping in the positive direction, -1 for negative direction
      #
      # e.g. 5 steps north is 5, :y, 1, 3 steps West is 3, :x, -1
      def take_steps(num_steps, coord: :x, direction: 1)
        while num_steps > 0
          case coord
          when :x
            @coords[0] += 1 * direction
          when :y
            @coords[1] += 1 * direction
          end

          record_position
          num_steps -= 1
        end
      end

      def record_position
        cur_coords = @coords.dup
        @visits[cur_coords] += 1

        @first_double_visit = cur_coords if @visits[cur_coords] > 1 && @first_double_visit.nil?
      end
    end
  end
end
