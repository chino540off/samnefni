extern crate serde;

mod cli;
mod command;
mod config;
mod model;

use cli::clap::CommandFactory;
use cli::clap::Parser;

fn main() {
    env_logger::init();

    let cli = cli::Parser::parse();
    let conf = config::Config::new(cli.config);

    match &cli.command {
        cli::Command::Exec { command, dry_run } => {
            let cmd = command::Command::new(command.to_vec());

            let cmd = match conf.find_aliases(&cmd.program) {
                Some(aliases) => cmd.resolve(aliases),
                None => cmd,
            };

            if !dry_run {
                cmd.execute();
            }
        }
        cli::Command::Completion { shell } => {
            let mut cmd = cli::Parser::command();
            cli::print_completions(shell.to_owned(), &mut cmd);
        }
    }
}
