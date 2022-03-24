use std::str::FromStr;

#[derive(Debug)]
struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum IsbnError {
    TooShort,
    TooLong,
    BadChecksum,
}

impl FromStr for Isbn {
    type Err = IsbnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.to_string();
        let mut digits = Vec::new();

        for ch in s.chars() {
            let digit = match ch {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => continue,
            };
            digits.push(digit);
        }

        if digits.len() < 13 {
            return Err(IsbnError::TooShort);
        } else if digits.len() > 13 {
            return Err(IsbnError::TooLong);
        }

        let given_check = digits.pop().unwrap();
        let computed_check = calculate_check_digit(digits.as_slice());

        if given_check != computed_check {
            Err(IsbnError::BadChecksum)
        } else {
            Ok(Isbn {raw, digits})
        }
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {

    let weighted_sum: u8 = digits
        .iter()
        .enumerate()
        .map(|(i, v)| if i % 2 == 0 {
            v * 1
        } else {
            v * 3
        })
        .sum();

    (10 - (weighted_sum % 10)) % 10
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
    println!("Its digits are {:?}.", rust_in_action.digits);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
