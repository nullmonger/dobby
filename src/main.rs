mod cli;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = cli::Cli::parse();
    eprintln!("dobby {}: not implemented yet", cli.command.verb());
    ExitCode::from(2)
}
