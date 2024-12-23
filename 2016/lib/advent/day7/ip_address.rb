module Advent
  module Day7
    class IPAddress
      def initialize(addr)
        @addr = addr
      end

      def supports_tls?
        hypernets = @addr.scan(/\[(\w+)\]/).map(&:first)
        return false if hypernets.any? { |h| has_abba? h }

        cleaned = @addr.dup
        hypernets.each { |h| cleaned.gsub!("[#{h}]", '') }
        has_abba? cleaned
      end

      def has_abba?(section)
        (0..section.length - 4).each do |i|
          return true if ((section[i] != section[i+1]) && (section[i] == section[i+3]) && (section[i+1] == section[i+2]))
        end

        false
      end
    end
  end
end
