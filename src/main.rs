extern crate clap;
use clap::{App, Arg};

const KEY_RANGE: u8 = 1 + ('Z' as u8) - ('A' as u8);

fn shift_alphabetic_char(c: char, n: u8) -> char {
    let offset = (c as u8) - ('A' as u8);
    let shift = (offset + n) % KEY_RANGE;
    ('A' as u8 + shift) as char
}

fn shift_char(c: char, n: u8) -> char {
    match c {
        _ if c.is_alphabetic() => shift_alphabetic_char(c, n),
        _ => c,
    }
}

pub fn shift(s: &String, n: u8) -> String {
    s.chars().map(|c| shift_char(c, n)).collect()
}

pub fn format_bruteforce_table(ciphered_message: &String) {
    for n in 0..KEY_RANGE {
        let s_shifted = shift(ciphered_message, n);
        println!("{:2} | {}", n, s_shifted);
    }
}

fn main() {
    let matches = App::new("Caesar")
        .version("0.1")
        .author("Joaquín P. Centeno")
        .about("Caesar cipher implementation.")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("INPUT")
                .help("Sets input text"),
        )
        .get_matches();

    let input = matches.value_of("input").expect("No input provided.");
    let input = String::from(input);

    format_bruteforce_table(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifting_empty_string_should_yield_empty_string() {
        let empty_string = String::from("");
        assert_eq!(shift(&empty_string, 0), empty_string);
        assert_eq!(shift(&empty_string, 5), empty_string);
        assert_eq!(shift(&empty_string, 14), empty_string);
    }

    #[test]
    fn shift_string_by_zero() {
        let input = String::from("FOO BAR BAZ XYZ");
        assert_eq!(shift(&input, 0), String::from("FOO BAR BAZ XYZ"));
    }

    #[test]
    fn shift_uppercase_string_by_one() {
        let input = String::from("ABC");
        let expected = String::from("BCD");
        assert_eq!(shift(&input, 1), expected);
    }

    #[test]
    fn shift_is_circular() {
        let input = String::from("XYZ");
        let expected = String::from("ABC");
        assert_eq!(shift(&input, 3), expected);
    }
}
