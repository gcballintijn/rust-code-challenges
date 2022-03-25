mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut result = String::new();
        let mut current = '\0';
        let mut count = 0;

        for ch in text.chars() {
            if count == 0 {
                current = ch;
                count = 1;
            } else if (current != ch) || (count == 9) {
                let pair = format!("{}{}", count, current);
                result.push_str(&pair);
                current = ch;
                count = 1;
            } else {
                count += 1;
            }
        }

        if count > 0 {
            let pair = format!("{}{}", count, current);
            result.push_str(&pair);
        }

        result
    }
    
    pub fn decode(text: &str) -> String {
        let mut result = String::new();
        let mut itr = text.chars();

        loop {
            let count = if let Some(digit) = itr.next() {
                digit.to_digit(10).unwrap()
            } else {
                break;
            };

            let ch = itr.next().unwrap();
            for _ in 0..count {
                result.push(ch);
            }         
        }

        result
    }
}

fn main() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("Encoded: {}", encode(input));

    let input = "1L1i1n1k1e1d1I1n";
    println!("Decoded: {}", decode(input));
}

#[cfg(test)]
mod test {
    use super::run_length_encoding::{encode, decode};

    #[test]
    fn encode_empty_string() {
        let input = "";
        assert_eq!(&encode(input), "");
    }

    #[test]
    fn encode_single_char() {
        let input = "X";
        assert_eq!(&encode(input), "1X");
    }

    #[test]
    fn encode_nine_x() {
        let input = "XXXXXXXXX";
        assert_eq!(&encode(input), "9X");
    }

    #[test]
    fn encode_ten_x() {
        let input = "XXXXXXXXXX";
        assert_eq!(&encode(input), "9X1X");
    }

    #[test]
    fn abc() {
        assert_eq!(encode("abc"), "1a1b1c");
    }

    #[test]
    fn long_run() {
        let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
        assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
    }

    #[test]
    fn decode_empty_string() {
        let input = "";
        assert_eq!(&decode(input), "");
    }

    #[test]
    fn decode_single_pair() {
        let input = "3X";
        assert_eq!(&decode(input), "XXX");
    }

    #[test]
    fn decode_two_pair_same() {
        let input = "9X1X";
        assert_eq!(&decode(input), "XXXXXXXXXX");
    }

    #[test]
    fn decode_two_pair_different() {
        let input = "3X4Y";
        assert_eq!(&decode(input), "XXXYYYY");
    }

    #[test]
    fn round_trip() {
        let input = "LinkedIn";
        assert_eq!(decode(&encode(input)), input);
    }

}
