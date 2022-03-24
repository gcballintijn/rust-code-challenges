use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb(u8, u8, u8);

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.0
    }

    fn g(&self) -> u8 {
        self.1
    }

    fn b(&self) -> u8 {
        self.2
    }
}

#[derive(Debug)]
enum RgbError {
    TooShort,
    TooLong,
    NoHash,
    InvalidLiteral,
}

impl FromStr for Rgb {
    type Err = RgbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 7 {
            return Err(RgbError::TooShort);
        } else if s.len() > 7 {
            return Err(RgbError::TooLong);
        }

        let mut iter = s.chars();
        if iter.next().unwrap() != '#' {
            return Err(RgbError::NoHash)
        }

        for ch in iter {
            match ch.to_ascii_lowercase() {
                '0'..='9' => {}
                'a'..='f' => {}
                _ => {
                    return Err(RgbError::InvalidLiteral)
                } 
            }            
        }

        let r: u8 = u8::from_str_radix(&s[1..3], 16).unwrap();
        let g: u8 = u8::from_str_radix(&s[3..5], 16).unwrap();
        let b: u8 = u8::from_str_radix(&s[5..7], 16).unwrap();

        Ok(Self(r, g, b))
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    // 
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short () {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}

