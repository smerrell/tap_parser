use regex::Regex;

pub fn read_version(input: &str) -> TapVersion {
    let re = Regex::new(r"^TAP version (?P<version>\d+)$").unwrap();

    if !re.is_match(&input) {
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

#[derive(Debug)]
pub struct TapParser {
    version: TapVersion,
    test_count: i32,
    total_tests: i32,
    failed_tests: i32,
}

impl TapParser {
    pub fn new(version: TapVersion) -> TapParser {
        TapParser {
            version: version,
            test_count: 0,
            total_tests: 0,
            failed_tests: 0,
        }
    }

    pub fn read_line(&mut self, line: &str) {
        let plan_re = Regex::new(r"^\d+..(?P<test_plan>\d+)$").unwrap();

        if plan_re.is_match(&line) {
            let test_plan = plan_re.captures(&line)
                .unwrap()
                .name("test_plan")
                .unwrap()
                .parse::<i32>()
                .unwrap();

            self.total_tests = test_plan;
        }

        let test_line = Regex::new(r"^(?P<failed>not )?ok - (?P<test_name>[^#]+)").unwrap();
        if test_line.is_match(&line) {

            let is_failed = test_line.captures(&line)
                .unwrap()
                .name("failed");

            match is_failed {
                Some(_) => self.failed_tests += 1,
                None => {},
            }
        }
    }

    pub fn summarize(&self) -> String {
        format!("{} tests ran, {} failed", &self.total_tests, &self.failed_tests).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parses_tap_version() {
        let input = "TAP version 13";
        let version = read_version(&input);
        assert_eq!(version, TapVersion::Thirteen);
    }

    #[test]
    pub fn parses_tap_version_twelve() {
        let input = "TAP version 12";
        let version = read_version(&input);
        assert_eq!(version, TapVersion::Twelve);
    }

    #[test]
    pub fn returns_unknown_when_version_number_unknown() {
        let input = "TAP version 123";
        let version = read_version(&input);
        assert_eq!(version, TapVersion::Unknown);
    }

    #[test]
    pub fn defaults_to_twelve_when_no_version_line() {
        let input = "";
        let version = read_version(&input);
        assert_eq!(version, TapVersion::Twelve);
    }

    #[test]
    #[should_panic(expected = "Can not declare TAP version below 12")]
    pub fn errors_when_version_below_12() {
        let input = "TAP version 11";
        let version = read_version(&input);
    }

    #[test]
    pub fn can_build_tap_parser() {
        let parser = TapParser::new(TapVersion::Thirteen);
        assert_eq!(parser.version, TapVersion::Thirteen);
    }

    #[test]
    pub fn returns_number_of_tests_from_plan_line() {
        let input = "1..14";
        let mut parser = TapParser::new(TapVersion::Thirteen);

        parser.read_line(&input);
        assert_eq!(parser.total_tests, 14);
    }

    #[test]
    pub fn tracks_number_of_failed_tests() {
        let input =
r"1..5
ok - Test the thing
ok - Test another thing
not ok - Test something broken
ok - Test again
not ok - Test another broken thing";
        let mut parser = TapParser::new(TapVersion::Thirteen);
        let lines = input.lines();
        for line in lines {
            println!("{}", line);
            parser.read_line(&line);
        }

        println!("{:?}", parser);
        assert_eq!(parser.failed_tests, 2);
    }

}
