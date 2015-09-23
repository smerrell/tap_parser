extern crate regex;

fn main() {
    // Need to be able to read from stdin
    // But we want most of the code to not have to rely on stdin

    println!("Hello, world!");

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
            panic!("We didn't get the TAP version")
        }

        let captures = re.captures(&input).unwrap();
        let version = captures.name("version").unwrap();

        match version {
            "13" => TapVersion::Thirteen,
            _ => TapVersion::Unknown
        }
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum TapVersion {
        Twelve,
        Thirteen,
        Unknown, // remove this, should default to Twelve
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
    pub fn returns_unknown_when_version_number_unknown() {
        let input = "TAP version 123";
        let version = tap::read_version(&input);
        assert_eq!(version, tap::TapVersion::Unknown);
    }

    #[test]
    #[ignore]
    pub fn defaults_to_twelve_when_no_version_line() {
    }

    #[test]
    #[ignore]
    pub fn errors_when_version_below_12() {
    }
}
