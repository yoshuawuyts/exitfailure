extern crate assert_cmd;
extern crate predicates;
use assert_cmd::prelude::*;
use predicates::prelude::*;

macro_rules! test_body {
    ($name:expr, $matcher:expr) => {
        let bin = assert_cmd::cargo::cargo_example_path($name).unwrap();
        let pred = predicates::str::contains($matcher).from_utf8();
        std::process::Command::new(bin)
            .assert()
            .failure()
            .stderr(pred);
    };
    ($name:expr) => {
        test_body!(
            $name,
            "Error: this is some context\nInfo: caused by root cause failure"
        )
    };
}

#[test]
fn test_example() {
    test_body!("example");
}

#[test]
fn test_context() {
    test_body!("context");
}
