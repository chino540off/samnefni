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

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Argument::Fold(arg) => write!(f, "{}...", arg),
            Argument::Simple(arg) => write!(f, "{}", arg),
        }
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

impl std::fmt::Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

pub type Aliases = std::collections::HashMap<String, Arguments>;
