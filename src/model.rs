pub type Program = String;

#[derive(Debug, PartialEq, Clone)]
pub enum Argument {
    Simple(String),
    Fold(String),
}

impl From<String> for Argument {
    fn from(s: String) -> Self {
        match s.find("...") {
            Some(i) => Argument::Fold(s.split_at(i).0.to_string()),
            None => Argument::Simple(s),
        }
    }
}

impl From<&str> for Argument {
    fn from(s: &str) -> Self {
        From::from(s.to_string())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Arguments(pub Vec<Argument>);

impl<'de> serde::Deserialize<'de> for Arguments {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Arguments(s.split(' ').map(From::from).collect()))
    }
}

pub type Aliases = std::collections::HashMap<String, Arguments>;
