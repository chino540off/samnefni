use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Alias {
    Simple { args: Vec<String> },
}

pub type Program = String;
pub type Aliases = std::collections::HashMap<String, Alias>;
