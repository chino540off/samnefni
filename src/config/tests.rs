extern crate rstest;

use self::rstest::*;
use super::*;

#[rstest]
fn path_default() {
    std::env::remove_var("SAMNEFNI_CONFIG");
    assert_eq!(
        config_path(None),
        std::path::PathBuf::from(std::env::var("HOME").unwrap() + "/.samnefni.toml")
    );
}

#[rstest]
fn path_set() {
    std::env::set_var("SAMNEFNI_CONFIG", "/samnefni.toml");
    assert_eq!(
        config_path(None),
        std::path::PathBuf::from("/samnefni.toml")
    );
    std::env::remove_var("SAMNEFNI_CONFIG");
}

#[fixture]
fn conf() -> Config {
    toml::from_str(
        r#"
            [aliases.cmd1]
                a = "add_c1"
                b = "bing_c1 --opt-c1 value"
            [aliases.cmd2]
                a = "add_c2"
                b = "bing_c2 --opt-c2 value"
            [aliases.cmd3]
                a = "add_c3 ..."
                b = "bing_c3 --opt-c3..."
                c = "call_c3 --opt-c3... --last"
        "#,
    )
    .unwrap()
}

#[rstest]
fn simple(conf: Config) {
    assert_eq!(
        conf.aliases.get("cmd1").unwrap().get("a").unwrap(),
        &model::Arguments(vec![model::Argument::Simple("add_c1".to_string())]),
    );
    assert_eq!(
        conf.aliases.get("cmd1").unwrap().get("b").unwrap(),
        &model::Arguments(vec![
            model::Argument::Simple("bing_c1".to_string()),
            model::Argument::Simple("--opt-c1".to_string()),
            model::Argument::Simple("value".to_string()),
        ]),
    );
    assert_eq!(
        conf.aliases.get("cmd2").unwrap().get("a").unwrap(),
        &model::Arguments(vec![model::Argument::Simple("add_c2".to_string()),]),
    );
    assert_eq!(
        conf.aliases.get("cmd2").unwrap().get("b").unwrap(),
        &model::Arguments(vec![
            model::Argument::Simple("bing_c2".to_string()),
            model::Argument::Simple("--opt-c2".to_string()),
            model::Argument::Simple("value".to_string()),
        ]),
    );
    assert_eq!(
        conf.aliases.get("cmd3").unwrap().get("a").unwrap(),
        &model::Arguments(vec![
            model::Argument::Simple("add_c3".to_string()),
            model::Argument::Fold("".to_string()),
        ]),
    );
    assert_eq!(
        conf.aliases.get("cmd3").unwrap().get("b").unwrap(),
        &model::Arguments(vec![
            model::Argument::Simple("bing_c3".to_string()),
            model::Argument::Fold("--opt-c3".to_string()),
        ]),
    );
    assert_eq!(
        conf.aliases.get("cmd3").unwrap().get("c").unwrap(),
        &model::Arguments(vec![
            model::Argument::Simple("call_c3".to_string()),
            model::Argument::Fold("--opt-c3".to_string()),
            model::Argument::Simple("--last".to_string()),
        ]),
    );
}
