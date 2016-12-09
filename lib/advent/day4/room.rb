module Advent
  module Day4
    class InvalidRoomSpec < RuntimeError; end

    # Room class for day 4 that checks validity of a roomspec
    class Room
      # Regex for pulling out the needed bits from the roomspec
      SPEC_RE = '((?:\w+\-)+\w+)\-(\d+)\[(\w+)\]'.freeze

      attr_reader :room, :sector, :checksum

      def initialize(roomspec)
        @debug = !ENV['DEBUG'].nil?
        @room, @sector, @checksum = roomspec.match(SPEC_RE).captures

        puts "Got Room #{@room} in sector #{@sector} with checksum #{@checksum}" if @debug
      rescue
        raise InvalidRoomSpec, "Could not parse roomspec! #{roomspec}"
      end

      def sector
        @sector.to_i
      end

      def calculated_checksum
        letters = @room.scan(/\w/)
        buckets = Hash.new { |h, v| h[v] = [] }

        # I could have sworn Ruby has a built-in for this...
        letter_counts = Hash.new 0
        letters.each { |l| letter_counts[l] += 1 }

        letter_counts.each do |k, v|
          buckets[v] << k
        end

        checksum = buckets.sort.reverse.map { |_, l| l.sort }.reduce(&:+).take(5).join
        puts "Calculated this checksum: #{checksum}" if @debug
        checksum
      end

      def valid?
        valid_checksum?
      end

      def valid_checksum?
        @checksum == calculated_checksum
      end
    end
  end
end
