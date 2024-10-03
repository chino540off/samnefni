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

#[rstest]
fn simple() {
    let conf: Config = toml::from_str(
        r#"
            [aliases.cmd1]
                a = { args = ["add_c1"] }
                b = { args = ["bing_c1", "--opt-c1", "value"] }
            [aliases.cmd2]
                a = { args = ["add_c2"] }
                b = { args = ["bing_c2", "--opt-c2", "value"] }
        "#,
    )
    .unwrap();

    assert_eq!(
        conf.aliases.get("cmd1").unwrap().get("a").unwrap(),
        &model::Alias::Simple {
            args: vec!["add_c1".to_string()]
        }
    );
    assert_eq!(
        conf.aliases.get("cmd1").unwrap().get("b").unwrap(),
        &model::Alias::Simple {
            args: vec![
                "bing_c1".to_string(),
                "--opt-c1".to_string(),
                "value".to_string()
            ]
        }
    );
    assert_eq!(
        conf.aliases.get("cmd2").unwrap().get("a").unwrap(),
        &model::Alias::Simple {
            args: vec!["add_c2".to_string()]
        }
    );
    assert_eq!(
        conf.aliases.get("cmd2").unwrap().get("b").unwrap(),
        &model::Alias::Simple {
            args: vec![
                "bing_c2".to_string(),
                "--opt-c2".to_string(),
                "value".to_string()
            ]
        }
    );
}
