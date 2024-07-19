use model;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub aliases: std::collections::HashMap<model::Program, model::Aliases>,
}

fn config_path() -> String {
    match std::env::var("SAMNEFNI_CONFIG") {
        Ok(path) => path,
        Err(_) => std::env!("HOME").to_owned() + "/.samnefni.toml",
    }
}

impl Config {
    pub fn new() -> Config {
        let filename = config_path();
        let contents = match std::fs::read_to_string(filename.clone()) {
            Ok(c) => c,
            Err(err) => {
                panic!("Could not read file `{}`: {}", filename, err);
            }
        };
        match toml::from_str::<Config>(&contents) {
            Ok(config) => config,
            Err(err) => {
                panic!("Unable to load data from `{}`: {}", filename, err);
            }
        }
    }
}
