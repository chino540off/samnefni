extern crate serde;

mod cli;
mod config;
mod model;

use cli::clap::CommandFactory;
use cli::clap::Parser;

fn main() {
    let conf = config::Config::new();
    let cli = cli::Parser::parse();

    match &cli.command {
        cli::Command::Exec { command } => {
            println!("{:?}", conf);
            println!("'exec' was used, command is: {command:?}");
        }
        cli::Command::Completion { shell } => {
            let mut cmd = cli::Parser::command();
            cli::print_completions(shell.clone(), &mut cmd);
        }
    }
}
