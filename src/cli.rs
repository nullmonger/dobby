use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Graft an export into a base file, keeping its order and formatting
    Merge,
    /// Show the structural delta between a base file and an export
    Diff,
    /// Rewrite a file in canonical form
    Format,
}

impl Command {
    pub fn verb(&self) -> &'static str {
        match self {
            Command::Merge => "merge",
            Command::Diff => "diff",
            Command::Format => "format",
        }
    }
}
