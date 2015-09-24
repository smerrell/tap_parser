extern crate regex;

use std::io::{self, BufRead};

fn main() {
    // Need to be able to read from stdin
    // But we want most of the code to not have to rely on stdin

    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();

    // this is probably bad, bad, bad
    let version_line = &iter.next().unwrap().unwrap();
    println!("{:?}", version_line);
    let version = tap::read_version(&version_line);
    println!("TAP Version: {:?}", version);

    for line in iter {
        println!("{}", line.unwrap().trim());
    }

    // What sort of types should we have?
    // How do you handle parsing a stream of text?
    //   Eventing of some kind?
    //   How do you do that in Rust?
    //
    // What sort of test cases are needed?
    // What sort of integration tests should there be?
}

pub mod tap {
    use regex::Regex;

    pub fn read_version(input: &str) -> TapVersion {
        let re = Regex::new(r"^TAP version (?P<version>\d+)$").unwrap();
        if !re.is_match(&input) {
            println!("no match");
            return TapVersion::Twelve
        }

        let captures = re.captures(&input).unwrap();
        let version = captures.name("version")
            .unwrap()
            .parse::<i32>()
            .unwrap();

        match version {
            0 ... 11 => panic!("Can not declare TAP version below 12"),
            12 => TapVersion::Twelve,
            13 => TapVersion::Thirteen,
            _ => TapVersion::Unknown
        }
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum TapVersion {
        Twelve,
        Thirteen,
        Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parses_tap_version() {
        let input = "TAP version 13";
        let version = tap::read_version(&input);
        assert_eq!(version, tap::TapVersion::Thirteen);
    }

    #[test]
    pub fn parses_tap_version_twelve() {
        let input = "TAP version 12";
        let version = tap::read_version(&input);
        assert_eq!(version, tap::TapVersion::Twelve);
    }

    #[test]
    pub fn returns_unknown_when_version_number_unknown() {
        let input = "TAP version 123";
        let version = tap::read_version(&input);
        assert_eq!(version, tap::TapVersion::Unknown);
    }

    #[test]
    pub fn defaults_to_twelve_when_no_version_line() {
        let input = "";
        let version = tap::read_version(&input);
        assert_eq!(version, tap::TapVersion::Twelve);
    }

    #[test]
    #[should_panic(expected = "Can not declare TAP version below 12")]
    pub fn errors_when_version_below_12() {
        let input = "TAP version 11";
        let version = tap::read_version(&input);
    }
}
