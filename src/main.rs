// use std::fmt::Display;
// fn info<T>(text: &T)
//     where T: Display
// {
//     println!("{}", text.to_string());
// }

// fn info<T>(text: &T)
//     where T: ToString
// {
//     println!("{}", text.to_string());
// }

// fn info<T>(text: &T)
//     where T: AsRef<str>
// {
//     println!("{}", text.as_ref());
// }

fn info<T>(text: &T)
    where T: Into<Vec<u8>> + Clone
{
    let x: Vec<u8> = text.to_owned().into();
    let s = String::from_utf8(x).unwrap();
    println!("{}", s);
}

fn main() {
    let a = "?";
    let b = "?".to_string();
    info(&a);
    info(&b);

    // Advanced 1
    use std::ffi::CString;    
    let c = CString::new("?").unwrap();
    info(&c);

    // Advanced 2
    // use std::path::Path;
    // let d = Path::new("/tmp/linkedin-learning");
    // info(&d);
}

#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}

// #[test]
// fn chars() {
//     let input = 'r';
//     info(&input);
// }

#[test]
fn cstring() {
    use std::ffi::{CString};
    let input = CString::new("Rust").unwrap();
    info(&input);
}

// #[test]
// fn path() {
//     use std::path::Path;
//     let input = Path::new("/tmp/rust");
//     info(&input);
// }
