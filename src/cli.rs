pub extern crate clap;

extern crate clap_complete;
extern crate shadow_rs;

use crate::command;
use crate::config;

use crate::cli::clap::CommandFactory;

shadow_rs::shadow!(build);

#[derive(clap::Parser)]
#[command(version(build::TAG), about, long_about = None)]
#[command(propagate_version = true)]
pub struct Parser {
    /// configuration path
    #[arg(long)]
    pub config: Option<std::path::PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Args)]
pub struct Execute {
    /// configuration path
    #[arg(long)]
    dry_run: bool,

    command: Vec<String>,
}

impl Execute {
    pub fn run(&self, conf: config::Config) {
        let cmd = command::Command::new(self.command.to_vec());

        let cmd = match conf.find_aliases(&cmd.program) {
            Some(aliases) => cmd.resolve(aliases),
            None => cmd,
        };

        if !self.dry_run {
            cmd.execute();
        }
    }
}

#[derive(Debug, clap::Args)]
pub struct Completion {
    /// target shell
    #[arg(long)]
    shell: clap_complete::Shell,

    /// Alias command
    command: Option<String>,
}

impl Completion {
    fn print_completions(&self, cmd: &mut clap::Command) {
        clap_complete::generate(
            self.shell,
            cmd,
            cmd.get_name().to_string(),
            &mut std::io::stdout(),
        );
    }

    pub fn run(&self, conf: config::Config) {
        let mut cmd = match &self.command {
            Some(command) => match conf.aliases.get(command) {
                Some(aliases) => clap::Command::new(format!("samnefni-{}", command))
                    .about(format!("run '{}' command", command))
                    .disable_help_flag(true)
                    .disable_help_subcommand(true)
                    .disable_version_flag(true)
                    .subcommands(
                        aliases
                            .iter()
                            .map(|(alias, arguments)| {
                                clap::Command::new(alias.clone())
                                    .about(format!("{}", arguments))
                                    .disable_help_flag(true)
                                    .disable_help_subcommand(true)
                                    .disable_version_flag(true)
                            })
                            .collect::<Vec<clap::Command>>(),
                    ),
                None => {
                    log::error!("no command found");
                    std::process::exit(1)
                }
            },
            None => Parser::command(),
        };
        self.print_completions(&mut cmd);
    }
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Execute a command and tries to bind aliases
    #[clap(aliases = &["e", "exec"])]
    Execute(Execute),

    /// Generate completion for a resquested shell
    #[clap(aliases = &["c", "comp"])]
    Completion(Completion),
}

impl Command {
    pub fn run(&self, conf: config::Config) {
        match self {
            Command::Execute(exec) => exec.run(conf),
            Command::Completion(completion) => completion.run(conf),
        }
    }
}
