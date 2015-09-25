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

pub struct tap_parser {
    version: TapVersion,
    test_count: i32,
    total_tests: i32,
}

impl tap_parser {
    fn new(version: TapVersion) -> tap_parser {
        tap_parser {
            version: version,
            test_count: 0,
            total_tests: 0,
        }
    }

    fn read_line(&mut self, line: &str) {
        let plan_re = Regex::new(r"$\d+..(?P<test_plan>\d+)").unwrap();
        if plan_re.is_match(&line) {
            let test_plan = plan_re.captures(&line)
                .unwrap()
                .name("test_plan")
                .unwrap()
                .parse::<i32>()
                .unwrap();

            println!("test_plan: {}", &test_plan);
            self.total_tests = test_plan;
        }
    }

    fn summarize(&self) -> String {
        format!("{} tests", &self.total_tests).to_string()
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
        let parser = tap_parser::new(TapVersion::Thirteen);
        assert_eq!(parser.version, TapVersion::Thirteen);
    }

    #[test]
    #[ignore]
    pub fn returns_number_of_tests_from_plan_line() {
        let input = "1..14";
        let mut parser = tap_parser::new(TapVersion::Thirteen);
        &mut parser.read_line(&input);
        let output = &mut parser.summarize();

        println!("output: {}", output);
        assert!(output.contains("14"));
    }

}
