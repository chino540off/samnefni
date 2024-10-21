extern crate serde;

mod cli;
mod command;
mod config;
mod model;

use cli::clap::Parser;

fn main() {
    env_logger::init();

    let cli = cli::Parser::parse();
    let conf = config::Config::new(cli.config);

    cli.command.run(conf);
}
