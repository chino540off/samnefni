extern crate serde;

mod command;
mod config;
mod model;

fn main() {
    let conf = config::Config::new();
    let cmd = command::Command::new(std::env::args().map(|arg| arg.to_string()).collect());

    match conf.find_aliases(&cmd.program) {
        Some(aliases) => cmd.resolve(aliases).execute(),
        None => cmd.execute(),
    };
}
