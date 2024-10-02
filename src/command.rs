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
mod tests {
    use crate::model;

    use super::Command;
    #[test]
    fn command() {
        assert_eq!(
            Command::new(vec![
                "cmd".to_string(),
                "sub-cmd".to_string(),
                "--opt".to_string(),
                "value".to_string()
            ]),
            Command {
                program: "cmd".to_string(),
                arguments: vec![
                    "sub-cmd".to_string(),
                    "--opt".to_string(),
                    "value".to_string()
                ],
            }
        );
    }

    #[test]
    fn resolve_simple() {
        let aliases = model::Aliases::from([(
            "r".to_string(),
            model::Alias::Simple {
                args: vec!["run".to_string(), "toto".to_string()],
            },
        )]);
        {
            let cmd = Command {
                program: "cmd".to_string(),
                arguments: vec!["r".to_string(), "--opt".to_string(), "value".to_string()],
            };
            assert_eq!(
                cmd.resolve(&aliases),
                Command {
                    program: "cmd".to_string(),
                    arguments: vec![
                        "run".to_string(),
                        "toto".to_string(),
                        "--opt".to_string(),
                        "value".to_string(),
                    ],
                }
            );
        }
        {
            let cmd = Command {
                program: "cmd".to_string(),
                arguments: vec!["e".to_string(), "--opt".to_string(), "value".to_string()],
            };
            assert_eq!(
                cmd.resolve(&aliases),
                Command {
                    program: "cmd".to_string(),
                    arguments: vec!["e".to_string(), "--opt".to_string(), "value".to_string(),],
                }
            );
        }
    }
}
