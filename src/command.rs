use crate::model;

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub program: model::Program,
    pub arguments: model::Arguments,
}

impl Command {
    pub fn new(arguments: model::Arguments) -> Command {
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
        let mut arguments: Vec<String> = vec![];

        for argument in &self.arguments {
            match aliases.get(argument) {
                Some(model::Alias::Simple { args }) => arguments.extend(args.clone()),
                None => arguments.push(argument.clone()),
            };
        }
        log::info!("{} {}", self.program, arguments.join(" "));

        Command {
            program: self.program.clone(),
            arguments,
        }
    }

    pub fn execute(&self) {
        match std::process::Command::new(self.program.clone())
            .args(self.arguments.clone())
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

#[cfg(test)]
mod tests;
