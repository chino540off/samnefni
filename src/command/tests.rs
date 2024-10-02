extern crate rstest;

use self::rstest::*;
use super::*;

#[fixture]
fn simple() -> model::Alias {
    model::Alias::Simple {
        args: vec!["run".to_string(), "toto".to_string()],
    }
}

#[fixture]
fn input() -> Command {
    Command {
        program: "cmd".to_string(),
        arguments: vec!["r".to_string(), "--opt".to_string(), "value".to_string()],
    }
}

#[rstest]
fn new() {
    assert_eq!(
        Command::new(vec![
            "cmd".to_string(),
            "sub-cmd".to_string(),
            "--opt".to_string(),
            "value".to_string()
        ]),
        Command {
            program: "cmd".to_string(),
            arguments: vec![
                "sub-cmd".to_string(),
                "--opt".to_string(),
                "value".to_string()
            ],
        }
    );
}

#[rstest]
#[case::unknown(("e".to_string(), simple()),
        Command {
            program: "cmd".to_string(),
            arguments: vec![
                "r".to_string(),
                "--opt".to_string(),
                "value".to_string(),
            ],
        }
    )]
#[case::simple(("r".to_string(), simple()),
        Command {
            program: "cmd".to_string(),
            arguments: vec![
                "run".to_string(),
                "toto".to_string(),
                "--opt".to_string(),
                "value".to_string(),
            ],
        }
    )]
fn resolve(input: Command, #[case] alias: (String, model::Alias), #[case] expected: Command) {
    let aliases = model::Aliases::from([alias]);
    assert_eq!(input.resolve(&aliases), expected);
}
