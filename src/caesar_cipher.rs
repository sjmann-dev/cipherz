fn get_char_numeric_equivalent(c: &char) -> i32 {
    match c  {
        'a' | 'A' => 0,
        'b' | 'B' => 1,
        'c' | 'C' => 2,
        'd' | 'D' => 3,
        'e' | 'E' => 4,
        'f' | 'F' => 5,
        'g' | 'G' => 6,
        'h' | 'H' => 7,
        'i' | 'I' => 8,
        'j' | 'J' => 9,
        'k' | 'K' => 10,
        'l' | 'L' => 11,
        'm' | 'M' => 12,
        'n' | 'N' => 13,
        'o' | 'O' => 14,
        'p' | 'P' => 15,
        'q' | 'Q' => 16,
        'r' | 'R' => 17,
        's' | 'S' => 18,
        't' | 'T' => 19,
        'u' | 'U' => 20,
        'v' | 'V' => 21,
        'w' | 'W' => 22,
        'x' | 'X' => 23,
        'y' | 'Y' => 24,
        'z' | 'Z' => 25,
        ' ' => 100,
        _ => panic!("Not a valid character!")
    }
}

fn get_numeric_char_equivalent(x: &i32) -> char {
    match x  {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        8 => 'i',
        9 => 'j',
        10 => 'k',
        11 => 'l',
        12 => 'm',
        13 => 'n',
        14 => 'o',
        15 => 'p',
        16 => 'q',
        17 => 'r',
        18 => 's',
        19 => 't',
        20 => 'u',
        21 => 'v',
        22 => 'w',
        23 => 'x',
        24 => 'y',
        25 => 'z',
        100 => ' ',
        _ => panic!("Not a valid numeric character representation!")
    }
}

pub enum Direction {
    Encode,
    Decode
}

pub fn caeser_cipher(text: &str, offset: i32, direction: Direction) -> String {
    let sanitised_offset = offset % 26;
    text.chars().map(|c| {
        let numeric_value = get_char_numeric_equivalent(&c);
        if numeric_value == 100 {
            return c;
        }
        let shifted_numeric_value = match direction {
            Direction::Encode => (numeric_value + sanitised_offset) % 26,
            Direction::Decode => {
                let raw_numeric = (numeric_value - sanitised_offset) % 26;
                if raw_numeric < 0 {
                    26 + raw_numeric
                } else {
                    raw_numeric
                }
            },
        };
        get_numeric_char_equivalent(&shifted_numeric_value)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caeser_cipher_with_offset_lt_26() {
        let input = "abcdef";
        let encoded = caeser_cipher(input, 1, Direction::Encode);
        let decoded = caeser_cipher(&encoded, 1, Direction::Decode);
        assert_eq!(encoded, "bcdefg");
        assert_eq!(decoded, input);
    }

    #[test]
    fn caeser_cipher_encode_with_offset_gt_26() {
        let input = "abcdef";
        let encoded = caeser_cipher(input, 27, Direction::Encode);
        let decoded = caeser_cipher(&encoded, 27, Direction::Decode);
        assert_eq!(encoded, "bcdefg");
        assert_eq!(decoded, input);
    }

    #[test]
    fn caeser_cipher_encode_handles_looping() {
        let input = "xyz";
        let encoded = caeser_cipher(input, 3, Direction::Encode);
        let decoded = caeser_cipher(&encoded, 3, Direction::Decode);
        assert_eq!(encoded, "abc");
        assert_eq!(decoded, input);
    }

    #[test]
    fn caeser_cipher_encode_preserves_spaces() {
        let input = "xyz abc";
        let encoded = caeser_cipher(input, 3, Direction::Encode);
        let decoded = caeser_cipher(&encoded, 3, Direction::Decode);
        assert_eq!(encoded, "abc def");
        assert_eq!(decoded, input);
    }
}