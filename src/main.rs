extern crate serde;

mod cli;
mod command;
mod config;
mod model;

use cli::clap::CommandFactory;
use cli::clap::Parser;

fn main() {
    let cli = cli::Parser::parse();
    let conf = config::Config::new(cli.config);

    match &cli.command {
        cli::Command::Exec { command } => {
            let cmd = command::Command::new(command.to_vec());

            match conf.find_aliases(&cmd.program) {
                Some(aliases) => cmd.resolve(aliases).execute(),
                None => cmd.execute(),
            };
        }
        cli::Command::Completion { shell } => {
            let mut cmd = cli::Parser::command();
            cli::print_completions(shell.to_owned(), &mut cmd);
        }
    }
}
