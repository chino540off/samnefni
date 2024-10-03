use serde::Deserialize;

pub type Program = String;
pub type Argument = String;
pub type Arguments = Vec<Argument>;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Alias {
    Simple { args: Arguments },
}

pub type Aliases = std::collections::HashMap<String, Alias>;
