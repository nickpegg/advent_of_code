require 'digest'

module Advent
  module Day5
    # Password generator for day 5
    class Password
      # Magic string that the md5sum starts with to indicate that the next
      # character of the password follows this string
      MAGIC_STRING = '00000'.freeze

      def initialize(door_id)
        @debug = !ENV['DEBUG'].nil?

        @door_id = door_id
        @md5 = Digest::MD5.new
        @id = 0
      end

      def generate
        password = ''
        digest = ''

        until password.length == 8
          digest = next_digest
          puts digest if @debug
          password << digest[MAGIC_STRING.length]
          digest = ''
        end

        password
      end

      private

      # Generate the next code for digesting
      def next_code
        code = @door_id + @id.to_s
        @id += 1
        code
      end

      def next_digest
        digest = ''
        digest = @md5.hexdigest next_code until digest_valid? digest
        digest
      end

      def digest_valid?(digest)
        digest.start_with? MAGIC_STRING
      end
    end
  end
end
