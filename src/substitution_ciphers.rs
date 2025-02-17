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

fn get_numeric_char_equivalent(x: i32) -> char {
    let sanitised_x = if x < 0 {26 + x} else {x};
    match sanitised_x  {
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

fn are_coprime(a: i32, b: i32) -> bool {
    gcd::binary_u32(a as u32, b as u32) == 1
}

pub enum Direction {
    Encode,
    Decode
}

pub fn caeser_cipher(plaintext: String, offset: i32, direction: Direction) -> Result<String, String> {
    affine_cipher(plaintext, 1, offset, direction)
}

pub fn atbash_cipher(plaintext: String, direction: Direction) -> Result<String, String> {
    affine_cipher(plaintext, -1, -1, direction)
}

pub fn rot_13_cipher(plaintext: String, direction: Direction) -> Result<String, String> {
    affine_cipher(plaintext, 1, 13, direction)
}

pub fn affine_cipher(plaintext: String, a: i32, b: i32, direction: Direction) -> Result<String, String> {
    if !are_coprime(a, 26) { return Err(format!("Value for a({a}) is not coprime with 26")); }
    Ok(plaintext.chars().map(|c| {
        let x = get_char_numeric_equivalent(&c);
        if x == 100 { return c; }
        let numeric_value = match direction {
            Direction::Encode => ((a * x) + b) % 26,
            Direction::Decode => (a as f64).powf(-1.0) as i32 * (x - b) % 26
        };
        get_numeric_char_equivalent(numeric_value)
    }).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caeser_cipher_with_offset_lt_26() {
        let input = "abcdef";
        let encoded = caeser_cipher(input.to_string(), 1, Direction::Encode);
        let decoded = caeser_cipher(encoded.clone().ok().unwrap().to_string(), 1, Direction::Decode);
        assert_eq!(encoded, Ok("bcdefg".to_string()));
        assert_eq!(decoded, Ok(input.to_string()));
    }

    #[test]
    fn caeser_cipher_with_offset_gt_26() {
        let input = "abcdef";
        let encoded = caeser_cipher(input.to_string(), 27, Direction::Encode);
        let decoded = caeser_cipher(encoded.clone().ok().unwrap(), 27, Direction::Decode);
        assert_eq!(encoded, Ok("bcdefg".to_string()));
        assert_eq!(decoded, Ok(input.to_string()));
    }

    #[test]
    fn rot_13_cipher_typical() {
        let input = "abcdef";
        let encoded = rot_13_cipher(input.to_string(), Direction::Encode);
        let decoded = rot_13_cipher(encoded.clone().ok().unwrap(), Direction::Decode);
        assert_eq!(encoded, Ok("nopqrs".to_string()));
        assert_eq!(decoded, Ok(input.to_string()));
    }

    #[test]
    fn caeser_cipher_handles_looping() {
        let input = "xyz";
        let encoded = caeser_cipher(input.to_string(), 3, Direction::Encode);
        let decoded = caeser_cipher(encoded.clone().ok().unwrap(), 3, Direction::Decode);
        assert_eq!(encoded, Ok("abc".to_string()));
        assert_eq!(decoded, Ok(input.to_string()));
    }

    #[test]
    fn caeser_cipher_preserves_spaces() {
        let input = "xyz abc";
        let encoded = caeser_cipher(input.to_string(), 3, Direction::Encode);
        let decoded = caeser_cipher(encoded.clone().ok().unwrap(), 3, Direction::Decode);
        assert_eq!(encoded, Ok("abc def".to_string()));
        assert_eq!(decoded, Ok(input.to_string()));
    }

    #[test]
    fn atbash_cipher_decode_and_encode() {
        let input = "abcdef";
        let encoded = atbash_cipher(input.to_string(), Direction::Encode);
        let decoded = atbash_cipher(encoded.clone().ok().unwrap(), Direction::Decode);
        assert_eq!(encoded, Ok("zyxwvu".to_string()));
        assert_eq!(decoded, Ok(input.to_string()));
    }

    #[test]
    fn atbash_cipher_preserves_spaces() {
        let input = "abc def";
        let encoded = atbash_cipher(input.to_string(), Direction::Encode);
        let decoded = atbash_cipher(encoded.clone().ok().unwrap(), Direction::Decode);
        assert_eq!(encoded, Ok("zyx wvu".to_string()));
        assert_eq!(decoded, Ok(input.to_string()));
    }

    #[test]
    fn affine_cipher_fails_when_coprime_constraint_violated() {
        let input = "abc def";
        let encoded = affine_cipher(input.to_string(), 2, 0, Direction::Encode);
        assert!(encoded.is_err());
        assert_eq!(encoded, Err(String::from("Value for a(2) is not coprime with 26")));
    }
}