#[cfg(test)]
extern crate hamcrest;
extern crate tap_parser;

use tap_parser::tap::{TapHarness};
use hamcrest::{assert_that,equal_to,is};

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
    assert_that(result.unwrap().passed, is(equal_to(false)));
}
