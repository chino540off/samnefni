use model;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub aliases: std::collections::HashMap<model::Program, model::Aliases>,
}

fn config_path(path: Option<std::path::PathBuf>) -> std::path::PathBuf {
    match (path, std::env::var("SAMNEFNI_CONFIG")) {
        (Some(path), _) => path,
        (None, Ok(path)) => std::path::PathBuf::from(path),
        (None, Err(_)) => {
            std::path::PathBuf::from(std::env!("HOME").to_owned() + "/.samnefni.toml")
        }
    }
}

impl Config {
    pub fn new(path: Option<std::path::PathBuf>) -> Config {
        let filename = config_path(path);
        let contents = match std::fs::read_to_string(filename.clone()) {
            Ok(c) => c,
            Err(err) => {
                panic!("Could not read file `{}`: {}", filename.display(), err);
            }
        };
        match toml::from_str::<Config>(&contents) {
            Ok(config) => config,
            Err(err) => {
                panic!("Unable to load data from `{}`: {}", filename.display(), err);
            }
        }
    }

    pub fn find_aliases(&self, program: &model::Program) -> Option<&model::Aliases> {
        self.aliases.get(program)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::{config::config_path, model};

    #[test]
    fn path_default() {
        std::env::remove_var("SAMNEFNI_CONFIG");
        assert_eq!(
            config_path(None),
            std::path::PathBuf::from(std::env::var("HOME").unwrap() + "/.samnefni.toml")
        );
    }

    #[test]
    fn path_set() {
        std::env::set_var("SAMNEFNI_CONFIG", "/samnefni.toml");
        assert_eq!(
            config_path(None),
            std::path::PathBuf::from("/samnefni.toml")
        );
        std::env::remove_var("SAMNEFNI_CONFIG");
    }

    #[test]
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
}
