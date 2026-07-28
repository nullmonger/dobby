mod cli;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = cli::Cli::parse();
    eprintln!("dobby: {:?} is not implemented yet", cli.command);
    ExitCode::from(2)
}
