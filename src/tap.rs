use regex::Regex;

#[derive(Debug)]
pub struct TapHarness {
    test_count: i32,
    total_tests: i32,
    failed_tests: i32,
    skipped_tests: i32,
    incomplete_tests: i32,
}

#[derive(Debug)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
}

impl TapHarness {
    pub fn new() -> TapHarness {
        TapHarness {
            test_count: 0,
            total_tests: 0,
            failed_tests: 0,
            skipped_tests: 0,
            incomplete_tests: 0,
        }
    }

    pub fn read_line(&mut self, line: &str) -> Option<TestResult> {
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

        let mut result = None;
        let test_line = Regex::new(r"^(?P<failed>not )?ok (?P<test_name>[^#]+)(# )?(?P<directive>\w+)?").unwrap();
        if test_line.is_match(&line) {
            self.test_count += 1;

            let directive = test_line.captures(&line).unwrap()
                .name("directive");
            let test_name = test_line.captures(&line).unwrap()
                .name("test_name");

            match directive {
                Some(d) => {
                    if d == "SKIP" {
                        self.skipped_tests += 1;
                    } else if d == "TODO" {
                        self.incomplete_tests += 1;
                    }
                },
                None => {
                    // Probably can do this a better way...
                    let is_failed = test_line.captures(&line).unwrap()
                        .name("failed");

                    let mut passed = true;
                    match is_failed {
                        Some(_) => {
                            self.failed_tests += 1;
                            passed = false;
                        },
                        None => {},
                    }

                    result = Some(TestResult {
                        name: test_name.unwrap().to_string(),
                        passed: passed,
                    });
                },
            }
        }

        result
    }

    pub fn summarize(&self) -> String {
        let mut failed_tests = self.failed_tests;
        if self.total_tests != self.test_count {
            failed_tests += self.total_tests - self.test_count;
        }

        format!("{} tests ran; {} failed; {} incomplete, {} skipped",
                &self.total_tests,
                failed_tests,
                &self.incomplete_tests,
                &self.skipped_tests).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn returns_number_of_tests_from_plan_line() {
        let input = "1..14";
        let mut parser = TapHarness::new();

        parser.read_line(&input);
        assert_eq!(parser.total_tests, 14);
    }

    #[test]
    pub fn tracks_number_of_failed_tests() {
        let input =
"1..5
ok 1 - Test the thing
ok 2 - Test another thing
not ok 3 - Test something broken
ok 4 - Test again
not ok 5 - Test another broken thing";
        let mut parser = TapHarness::new();
        let lines = input.lines();
        for line in lines {
            println!("{}", line);
            parser.read_line(&line);
        }

        println!("{:?}", parser);
        assert_eq!(parser.failed_tests, 2);
    }

    #[test]
    pub fn missing_test_counted_as_failed() {
        let input =
"1..5
ok - Test the thing
not ok - Test something broken
ok - Test again";
        let mut parser = TapHarness::new();
        let lines = input.lines();
        for line in lines {
            println!("{}", line);
            parser.read_line(&line);
        }

        let output = parser.summarize();
        println!("{}", output);
        println!("{:?}", parser);
        assert!(output.contains("3 failed"));
    }

    #[test]
    pub fn skipped_test_not_considered_failed() {
        let input =
"1..5
ok 1 - Test the thing # SKIP no foobaz available
ok 2 - Test another thing # SKIP no foobaz available
not ok 3 - Test something broken # SKIP no bar
ok 4 - Test again
not ok 5 - Test another broken thing";
        let mut parser = TapHarness::new();
        let lines = input.lines();
        for line in lines {
            parser.read_line(&line);
        }

        println!("{:?}", parser);
        assert_eq!(parser.skipped_tests, 3);
        assert_eq!(parser.failed_tests, 1)
    }

    #[test]
    pub fn todo_tests_not_considered_failed() {
        let input =
"1..5
ok 1 - Test the thing # TODO finish this test
ok 2 - Test another thing # TODO finish this test
not ok 3 - Test something broken # TODO finish this test
ok 4 - Test again
not ok 5 - Test another broken thing";
        let mut parser = TapHarness::new();
        let lines = input.lines();
        for line in lines {
            parser.read_line(&line);
        }

        println!("{:?}", parser);
        assert_eq!(parser.failed_tests, 1)
    }

    #[test]
    pub fn todo_tests_counted_as_incomplete_tests() {
        let input =
"1..5
ok 1 - Test the thing # TODO finish this test
ok 2 - Test another thing # TODO finish this test
not ok 3 - Test something broken # TODO finish this test
ok 4 - Test again
not ok 5 - Test another broken thing";
        let mut parser = TapHarness::new();
        let lines = input.lines();
        for line in lines {
            parser.read_line(&line);
        }

        println!("{:?}", parser);
        assert_eq!(parser.incomplete_tests, 3);
        assert_eq!(parser.failed_tests, 1)
    }

    #[test]
    pub fn read_line_for_failed_test_returns_passed_false() {
        let input =
"1..5
not ok Test something broken";
        let mut parser = TapHarness::new();
        let mut lines = input.lines();
        parser.read_line(&lines.next().unwrap());
        let result = parser.read_line(&lines.next().unwrap());

        println!("{:?}", result);
        assert_eq!(result.unwrap().passed, false);
    }
    // TODO: Probably break some of these out into "integration" tests in their own file
    // TODO: What is the exact format for tap output? Is there supposed to be a dash or not?

}
