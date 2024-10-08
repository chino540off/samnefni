extern crate rstest;

use self::rstest::*;
use super::*;

#[fixture]
fn simple() -> model::Arguments {
    model::Arguments(vec![
        model::Argument::Simple("run".to_string()),
        model::Argument::Simple("toto".to_string()),
    ])
}

#[fixture]
fn fold_option() -> model::Arguments {
    model::Arguments(vec![
        model::Argument::Simple("apply".to_string()),
        model::Argument::Fold("-f".to_string()),
        model::Argument::Simple("--last".to_string()),
    ])
}

#[fixture]
fn fold_no_option() -> model::Arguments {
    model::Arguments(vec![
        model::Argument::Simple("apply".to_string()),
        model::Argument::Fold("".to_string()),
        model::Argument::Simple("--last".to_string()),
    ])
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
#[case::unknown(
        ("e".to_string(), simple()),
        Command::from("cmd r --opt value"),
        Command::from("cmd r --opt value"),
    )]
#[case::simple(
        ("r".to_string(), simple()),
        Command::from("cmd r --opt value"),
        Command::from("cmd run toto --opt value"),
    )]
#[case::fold_option(
        ("a".to_string(), fold_option()),
        Command::from("cmd a f1 f2"),
        Command::from("cmd apply -f f1 -f f2 --last"),
    )]
#[case::fold_no_option(
        ("a".to_string(), fold_no_option()),
        Command::from("cmd a f1 f2"),
        Command::from("cmd apply f1 f2 --last"),
    )]
fn resolve(
    #[case] alias: (String, model::Arguments),
    #[case] input: Command,
    #[case] expected: Command,
) {
    let aliases = model::Aliases::from([alias]);
    assert_eq!(input.resolve(&aliases), expected);
}
