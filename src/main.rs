use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

/// Represents a single character
type Letter = Vec<Pulse>;

/// Represents a string of characters
type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        let mut result = Vec::new();
        for ch in self.to_lowercase().chars() {
            match to_morse_code_letter(ch) {
                Some(l) => result.push(l),
                None => (),
            }
        }
        result
    }
}

fn to_morse_code_letter(ch: char) -> Option<Letter> {
    use Pulse::*;

    match ch {
        'h' => Some(vec![Short, Short, Short, Short]),
        'e' => Some(vec![Short]),
        'l' => Some(vec![Short, Long, Short, Short]),
        'o' => Some(vec![Long, Long, Long]),
        'w' => Some(vec![Short, Long, Long]),
        'r' => Some(vec![Short, Long, Short]),
        'd' => Some(vec![Long, Short, Short]),
        _ => None            
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        };
        print!(" ");
    };
    println!();
}

fn main() {
    let greeting = "Hello, world"
        .to_string()
        .to_morse_code();
    
    print_morse_code(&greeting);
}

#[test]
fn hello_world() {
    use Pulse::*;

    let expected = vec![
        vec![Short, Short, Short, Short],
        vec![Short],
        vec![Short, Long, Short, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Long, Long],
        vec![Short, Long, Long],
        vec![Long, Long, Long],
        vec![Short, Long, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Short, Short],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

    alphabet.to_morse_code();
    alphabet.to_uppercase().to_morse_code();
}
