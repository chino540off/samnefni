pub extern crate clap;

extern crate clap_complete;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Parser {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Execute a command and tries to bind aliases
    Exec { command: Vec<String> },
    /// Generate completion for a resquested shell
    Completion { shell: clap_complete::Shell },
}

pub fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
