use regex::Regex;

#[derive(Debug)]
pub struct TapHarness {
    test_count: i32,
    total_tests: i32,
    failed_tests: i32,
    skipped_tests: i32,
    incomplete_tests: i32,
    diagnostics: Vec<String>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct TestResult {
    pub name: String,
    pub state: TestState,
    pub diagnostics: Option<Vec<String>>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TestState {
    Passed,
    Failed,
    Skipped,
    Incomplete,
}

impl TapHarness {
    pub fn new() -> TapHarness {
        TapHarness {
            test_count: 0,
            total_tests: 0,
            failed_tests: 0,
            skipped_tests: 0,
            incomplete_tests: 0,
            diagnostics: Vec::new(),
        }
    }

    fn handle_test_plan(&mut self, line: &str) {
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
    }

    fn handle_diagnostic_line(&mut self, line: &str) {
        let diagnostic_re = Regex::new(r"^# (?P<diagnostic>.*)").unwrap();
        if diagnostic_re.is_match(&line) {
            let message = diagnostic_re.captures(&line)
                                       .unwrap()
                                       .name("diagnostic")
                                       .unwrap();

            self.diagnostics.push(message.to_owned());
        }
    }

    fn handle_directive(&mut self, directive: &str, test_name: Option<&str>) -> Option<TestResult> {
        let mut test_result = TestState::Passed;
        if directive == "SKIP" {
            self.skipped_tests += 1;
            test_result = TestState::Skipped;
        } else if directive == "TODO" {
            self.incomplete_tests += 1;
            test_result = TestState::Incomplete;
        }

        Some(TestResult {
            name: test_name.unwrap().to_string(),
            state: test_result,
            diagnostics: None,
        })
    }

    pub fn read_line(&mut self, line: &str) -> Option<TestResult> {

        self.handle_test_plan(&line);
        self.handle_diagnostic_line(&line);

        let mut result = None;
        let test_line = test_line_regex();
        if test_line.is_match(&line) {
            self.test_count += 1;

            let test_name = test_line.captures(&line)
                                     .unwrap()
                                     .name("test_name");
            let directive = test_line.captures(&line)
                                     .unwrap()
                                     .name("directive");

            let mut test_result = TestState::Passed;
            match directive {
                Some(d) => {
                    result = self.handle_directive(d, test_name);
                }
                None => {
                    // Probably can do this a better way...
                    let is_failed = test_line.captures(&line)
                                             .unwrap()
                                             .name("failed");

                    match is_failed {
                        Some(_) => {
                            self.failed_tests += 1;
                            test_result = TestState::Failed;
                        }
                        None => {
                            self.diagnostics.clear();
                        }
                    }

                    // print all diagnostic lines if we've failed
                    result = Some(TestResult {
                        name: test_name.unwrap().to_string(),
                        diagnostics: match &test_result {
                            &TestState::Failed => Some(self.diagnostics.drain(..).collect()),
                            _ => None,
                        },
                        state: test_result,
                    });
                }
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
                &self.skipped_tests)
            .to_string()
    }
}

fn test_line_regex() -> Regex {
    Regex::new(r"^(?P<failed>not )?ok ?(?P<test_number>\d*)? ?(?P<test_name>[^#]+)?(# )?(?P<directive>\w+)?").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_line_regex;
    use hamcrest::{assert_that, equal_to, is};

    macro_rules! regex_matches{
        ($regex:expr, $line:expr, $matches:expr) => {{
            println!("'{}' - checking matches", $line);
            assert_that($regex.is_match($line), is(equal_to($matches)));
        }}
    }

    macro_rules! regex_match_group_equals {
        ($regex:expr, $line:expr, $groupname:expr, $matches:expr) => {{

            println!("checking group '{}' equals '{}'", $groupname, $matches);
            let group_value = $regex.captures($line)
                               .unwrap()
                               .name($groupname)
                               .unwrap();
            assert_eq!(group_value, $matches);
        }}
    }

    #[test]
    pub fn handles_valid_tap_test_lines() {
        let regex = test_line_regex();
        regex_matches!(regex, "ok", true);
        regex_matches!(regex, "ok # skip", true);
        regex_matches!(regex, "not ok", true);
        regex_matches!(regex, "not ok # skip", true);
        regex_matches!(regex, "ok 1", true);
        regex_matches!(regex, "ok 1 # skip", true);
        regex_matches!(regex, "not ok 1 # skip", true);
        regex_matches!(regex, "not ok 1 this is a test name", true);
        regex_matches!(regex,
                       "ok 1 this is a skipped test # SKIP no db available",
                       true);
        regex_matches!(regex, "ok this is a skipped test # TODO not finished", true);
    }

    #[test]
    pub fn match_groups_work_properly_for_test_lines() {
        let regex = test_line_regex();
        regex_match_group_equals!(regex, "ok 1", "test_number", "1");
        regex_match_group_equals!(regex, "not ok 1", "failed", "not ");
        regex_match_group_equals!(regex, "ok # skip", "directive", "skip");
        regex_match_group_equals!(regex, "ok test name # skip", "test_name", "test name ");
    }

    #[test]
    pub fn returns_number_of_tests_from_plan_line() {
        let input = "1..14";
        let mut parser = TapHarness::new();

        parser.read_line(&input);
        assert_that(parser.total_tests, is(equal_to(14)));
    }

    #[test]
    pub fn handles_all_tests_skipped() {
        let input = "1..0";
        let mut parser = TapHarness::new();

        parser.read_line(&input);
        assert_that(parser.total_tests, is(equal_to(0)));
    }

    #[test]
    pub fn tracks_number_of_failed_tests() {
        let input = "1..5
ok 1 - Test the thing
ok 2 - Test another thing
not ok 3 - Test \
                     something broken
ok 4 - Test again
not ok 5 - Test another broken thing";
        let mut parser = TapHarness::new();
        let lines = input.lines();
        for line in lines {
            parser.read_line(&line);
        }

        assert_that(parser.failed_tests, is(equal_to(2)));
    }

    #[test]
    pub fn missing_test_counted_as_failed() {
        let input = "1..5
ok - Test the thing
not ok - Test something broken
ok - Test again";
        let mut parser = TapHarness::new();
        let lines = input.lines();
        for line in lines {
            parser.read_line(&line);
        }

        let output = parser.summarize();
        assert!(output.contains("3 failed"));
    }

    #[test]
    pub fn skipped_test_not_considered_failed() {
        let input = "1..5
ok 1 - Test the thing # SKIP no foobaz available
ok 2 - Test another \
                     thing # SKIP no foobaz available
not ok 3 - Test something broken # SKIP no \
                     bar
ok 4 - Test again
not ok 5 - Test another broken thing";
        let mut parser = TapHarness::new();
        let lines = input.lines();
        for line in lines {
            parser.read_line(&line);
        }

        assert_that(parser.skipped_tests, is(equal_to(3)));
        assert_that(parser.failed_tests, is(equal_to(1)));
    }

    #[test]
    pub fn todo_tests_not_considered_failed() {
        let input = "1..5
ok 1 - Test the thing # TODO finish this test
ok 2 - Test another thing \
                     # TODO finish this test
not ok 3 - Test something broken # TODO finish this \
                     test
ok 4 - Test again
not ok 5 - Test another broken thing";
        let mut parser = TapHarness::new();
        let lines = input.lines();
        for line in lines {
            parser.read_line(&line);
        }

        assert_that(parser.failed_tests, is(equal_to(1)));
    }

    #[test]
    pub fn todo_tests_counted_as_incomplete_tests() {
        let input = "1..5
ok 1 - Test the thing # TODO finish this test
ok 2 - Test another thing \
                     # TODO finish this test
not ok 3 - Test something broken # TODO finish this \
                     test
ok 4 - Test again
not ok 5 - Test another broken thing";
        let mut parser = TapHarness::new();
        let lines = input.lines();
        for line in lines {
            parser.read_line(&line);
        }

        assert_that(parser.incomplete_tests, is(equal_to(3)));
        assert_that(parser.failed_tests, is(equal_to(1)));
    }

    #[test]
    pub fn read_line_for_failed_test_returns_passed_false() {
        let input = "1..5
not ok Test something broken";
        let mut parser = TapHarness::new();
        let mut lines = input.lines();
        parser.read_line(&lines.next().unwrap());
        let result = parser.read_line(&lines.next().unwrap());

        assert_that(result.unwrap().state, is(equal_to(TestState::Failed)));
    }
    // TODO: What is the exact format for tap output? Is there supposed to be a dash or not?
}
