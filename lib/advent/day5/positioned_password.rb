require 'advent/day5/password'
require 'digest'

module Advent
  module Day5
    # Class that fills in the password where the digit after the magic string
    # is the position in the password to fill in the character, and the
    # character after _that_ is the character to fill in with
    class PositionedPassword < Password
      VALID_POSITIONS = (0..7).to_a.map(&:to_s)

      def initialize(door_id)
        super(door_id)
      end

      def generate
        password = [nil] * 8
        digest = ''

        until filled_in? password
          digest = next_digest
          puts digest if @debug

          pos = digest[MAGIC_STRING.length].to_i
          next unless password[pos].nil?

          password[pos] = digest[MAGIC_STRING.length + 1]
          puts "Putting #{digest[MAGIC_STRING.length + 1]} into position #{pos}" if @debug

          digest = ''
        end

        password.join
      end

      private

      def digest_valid?(digest)
        digest.start_with?(MAGIC_STRING) && VALID_POSITIONS.include?(digest[MAGIC_STRING.length])
      end

      def filled_in?(password)
        !password.include? nil
      end
    end
  end
end
