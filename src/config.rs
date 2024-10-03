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
mod tests;
