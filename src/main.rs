extern crate pest;
extern crate pest_derive;
extern crate serde;

mod cli;
mod command;
mod config;
mod model;

use cli::clap::CommandFactory;
use cli::clap::Parser;
use pest::Parser as pestParser;

#[derive(pest_derive::Parser)]
#[grammar_inline = r#"
// your grammar here
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }

char = {
    !("\"" | "\\") ~ ANY
    | "\\" ~ ("\"" | "\\" | "/" | "b" | "f" | "n" | "r" | "t")
    | "\\" ~ ("u" ~ ASCII_HEX_DIGIT{4})
}

argument = {
    "-" ~ char+
    | char+
}

command = { SOI ~ argument* ~EOI }
"#]
struct AliasParser;

fn main() {
    let parse_result = AliasParser::parse(Rule::command, "run -it --rm").expect("fail");
    let tokens = parse_result.tokens();

    for token in tokens {
        println!("{:?}", token);
    }

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
