use std::iter::FromIterator;
use crate::alphabet::{Alphabet, Classic};

pub fn encode(data: &[u8]) -> String {
    let classic_alphabet = &Classic {};
    encode_using_alphabet(classic_alphabet, data)
}

pub fn encode_using_alphabet<T: Alphabet>(alphabet: &T, data: &[u8]) -> String {
    let encoded = data
        .chunks(3)
        .flat_map(|chunk| encode_chunk(alphabet, chunk));

    String::from_iter(encoded)
}

fn encode_chunk<T: Alphabet>(alphabet: &T, chunk: &[u8]) -> Vec<char> {
    match chunk.len() {
        1 => vec![
            char_for_index(alphabet, first(&chunk[0])), 
            char_for_index(alphabet, second(&chunk[0], &0)),
            '=', 
            '='
        ],

        2 => vec![
            char_for_index(alphabet, first(&chunk[0])),
            char_for_index(alphabet, second(&chunk[0], &chunk[1])),
            char_for_index(alphabet, third(&chunk[1], &0)),
            '='
        ],

        3 => vec![
            char_for_index(alphabet, first(&chunk[0])),
            char_for_index(alphabet, second(&chunk[0], &chunk[1])),
            char_for_index(alphabet, third(&chunk[1], &chunk[2])),
            char_for_index(alphabet, fourth(&chunk[2])),
        ],

        _ => unreachable!("index cannot be outside of 0 through 2"),
    }
}

fn first(byte: &u8) -> u8 {
    byte >> 2
}

fn second(byte: &u8, second: &u8) -> u8 {
    // take the last two bits, shift them 4 positions left
    let part_one = (byte & 0b00000011) << 4;

    // take the first four bits
    let part_two = second >> 4;

    part_one | part_two
}

fn third(first: &u8, second: &u8) -> u8 {
    // take the last two bits, shift them 4 positions left
    let part_one = (first & 0b00001111) << 2;

    // take the first four bits
    let part_two = second >> 6;

    part_one | part_two
}

fn fourth(byte: &u8) -> u8 {
    byte & 0b00111111
}

fn char_for_index<T: Alphabet>(alphabet: &T, index: u8) -> char {
    alphabet
        .get_char_for_index(index)
        .expect("char index not in alphabet")
}

#[cfg(test)]
mod tests {
    use super::encode;

    #[test]
    fn test_single_char() {
        let input_str = "a";
        let expected = "YQ==";

        let input_data = input_str.as_bytes();

        assert_eq!(encode(input_data), expected);
    }

    #[test]
    fn test_two_chars() {
        let input_str = "ab";
        let expected = "YWI=";

        let input_data = input_str.as_bytes();

        assert_eq!(encode(input_data), expected);
    }

    #[test]
    fn test_three_chars() {
        let input_str = "abc";
        let expected = "YWJj";

        let input_data = input_str.as_bytes();

        assert_eq!(encode(input_data), expected);
    }

    #[test]
    fn tests_short_string() {
        let input_str = "Hello, world!";
        let expected = "SGVsbG8sIHdvcmxkIQ==";

        let input = input_str.as_bytes();

        assert_eq!(encode(input), expected);
    }

    #[test]
    fn test_longer_string() {
        let input_str = "And here be a bit longer text. Let's see how it goes!";
        let expected = "QW5kIGhlcmUgYmUgYSBiaXQgbG9uZ2VyIHRleHQuIExldCdzIHNlZSBob3cgaXQgZ29lcyE=";

        let input_data = input_str.as_bytes();

        assert_eq!(encode(input_data), expected);
    }
}
