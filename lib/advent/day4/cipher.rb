module Advent
  module Day4
    class Cipher
      ALPHABET = ('a'.ord..'z'.ord).map(&:chr)

      def self.decrypt(text, rotations)
        decrypted = ''
        text.split('').each do |char|
          if self.is_letter char
            decrypted << rotate(char, rotations)
          else
            decrypted << char
          end
        end

        decrypted
      end

      private
      def self.is_letter(char)
        char.ord >= 'a'.ord && char.ord <= 'z'.ord
      end

      def self.rotate(letter, offset)
        pos = ALPHABET.find_index letter
        ALPHABET[(pos + offset) % ALPHABET.length]
      end
    end
  end
end
