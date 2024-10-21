use crate::model;

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub program: model::Program,
    pub arguments: Vec<String>,
}

fn expand_arguments(
    alias_args: &Vec<model::Argument>,
    arg_iter: &mut std::slice::Iter<'_, String>,
) -> Vec<String> {
    let mut arguments: Vec<String> = vec![];

    for alias_arg in alias_args {
        match alias_arg {
            model::Argument::Simple(value) => arguments.push(value.to_string()),
            model::Argument::Fold(value) => {
                for arg in arg_iter.into_iter() {
                    if !value.is_empty() {
                        arguments.push(value.to_string());
                    }
                    arguments.push(arg.to_string());
                }
            }
        }
    }
    arguments.extend(
        arg_iter
            .as_slice()
            .iter()
            .map(|arg| arg.clone())
            .collect::<Vec<_>>(),
    );
    arguments
}

impl Command {
    pub fn new(arguments: Vec<String>) -> Command {
        let program = &arguments[0..1][0];
        Command {
            program: program.to_string(),
            arguments: arguments[1..]
                .iter()
                .map(|argument| argument.to_string())
                .collect(),
        }
    }

    pub fn resolve(&self, aliases: &model::Aliases) -> Command {
        let mut arguments = vec![];
        let mut arg_iter = self.arguments.iter();

        if let Some(arg) = arg_iter.next() {
            arguments = match aliases.get(arg) {
                Some(model::Arguments(alias_args)) => expand_arguments(&alias_args, &mut arg_iter),
                None => self.arguments.clone(),
            }
        }

        let result = Command {
            program: self.program.clone(),
            arguments,
        };
        log::debug!("'{}' -> '{}'", self, result);
        result
    }

    pub fn execute(&self) {
        log::info!("execute '{}'", self);
        match std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("{}", self))
            .envs(std::env::vars())
            .spawn()
        {
            Ok(mut child) => {
                child.wait().expect("failed to wait child");
            }
            Err(err) => {
                log::error!("{}", err);
            }
        };
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.program, self.arguments.join(" "))
    }
}

impl std::convert::From<&str> for Command {
    fn from(s: &str) -> Self {
        Self::new(s.split(" ").map(|s| s.to_string()).collect())
    }
}

#[cfg(test)]
mod tests;
