//! The command line interface for Pikelet

extern crate failure;
extern crate pikelet_language_server;
extern crate pikelet_repl;
#[macro_use]
extern crate structopt;

use failure::Error;

// TODO: test using https://github.com/killercup/assert_cli

#[derive(Debug, StructOpt)]
#[structopt(name = "pikelet")]
pub struct Opts {
    /// Subcommand to run
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// A REPL for running expressions
    #[structopt(name = "repl")]
    Repl(pikelet_repl::Opts),
    /// Start an instance of the the language server
    #[structopt(name = "language-server")]
    LanguageServer(pikelet_language_server::Opts),
}

/// Run `pikelet` with the given options
pub fn run(opts: Opts) -> Result<(), Error> {
    match opts.command {
        Command::LanguageServer(opts) => pikelet_language_server::run(opts),
        Command::Repl(opts) => pikelet_repl::run(opts),
    }
}
