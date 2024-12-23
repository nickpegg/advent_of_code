module Advent
  module Day4
    # Caesar Cipher
    class Cipher
      ALPHABET = ('a'.ord..'z'.ord).map(&:chr)

      def self.decrypt(text, rotations)
        decrypted = ''

        text.downcase.split('').each do |char|
          decrypted << if letter?(char)
                         rotate(char, rotations)
                       else
                         char
                       end
        end

        decrypted
      end

      def self.letter?(char)
        char.ord >= 'a'.ord && char.ord <= 'z'.ord
      end

      def self.rotate(letter, offset)
        pos = ALPHABET.find_index letter
        ALPHABET[(pos + offset) % ALPHABET.length]
      end
    end
  end
end
