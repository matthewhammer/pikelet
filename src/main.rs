extern crate lambdapi;
extern crate rustyline;
#[macro_use]
extern crate structopt;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::PathBuf;
use structopt::StructOpt;

use lambdapi::semantics;
use lambdapi::syntax::parse;

const PROMPT: &str = "λΠ> ";
const REPL_HISTORY_FILE: &str = "repl-history";

#[derive(StructOpt, Debug)]
#[structopt(name = "lambdapi")]
enum Opt {
    /// Check the that the give files type check
    #[structopt(name = "check")]
    Check {
        /// Files to check
        #[structopt(name = "FILE", parse(from_os_str))]
        files: Vec<PathBuf>,
    },

    /// A REPL for running expressions
    #[structopt(name = "repl")]
    Repl {
        /// Files to preload into the REPL
        #[structopt(name = "FILE", parse(from_os_str))]
        files: Vec<PathBuf>,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Check { files } => run_check(files),
        Opt::Repl { files } => run_repl(files),
    }
}

fn run_check(_files: Vec<PathBuf>) {
    // TODO: Load files

    unimplemented!()
}

fn run_repl(_files: Vec<PathBuf>) {
    // TODO: Load files

    let mut rl = Editor::<()>::new();

    if let Err(_) = rl.load_history(REPL_HISTORY_FILE) {}

    println!(
        "{}, version {} (:? for help, :q to quit)",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    loop {
        match rl.readline(PROMPT) {
            Ok(line) => {
                rl.add_history_entry(&line);

                match step_repl(&line) {
                    Ok(()) => {},
                    Err(ReplError::Parse(err)) => println!("parse error: {}", err),
                    Err(ReplError::Type(err)) => println!("type error: {:?}", err),
                    Err(ReplError::Quit) => {
                        println!("Bye bye");
                        break;
                    },
                }
            },
            Err(err) => match err {
                ReadlineError::Interrupted => println!("Interrupt"),
                ReadlineError::Eof => {
                    println!("Bye bye");
                    break;
                },
                err => {
                    println!("Error: {:?}", err);
                    break;
                },
            },
        }
    }

    rl.save_history(REPL_HISTORY_FILE).unwrap();
}

fn step_repl(line: &str) -> Result<(), ReplError> {
    use lambdapi::semantics;
    use lambdapi::syntax::concrete::ReplCommand;
    use lambdapi::syntax::core::{Context, RcTerm};
    use lambdapi::syntax::pretty::{self, ToDoc};
    use lambdapi::syntax::translation::FromConcrete;

    match line.parse()? {
        ReplCommand::Help => {
            println!("");
            println!("Command       Arguments   Purpose");
            println!("");
            println!("<expr>                    evaluate a term");
            println!(":? :h :help               display this help text");
            println!(":q :quit                  quit the repl");
            println!(":t :type      <expr>      infer the type of an expression");
            println!("");
        },

        ReplCommand::Eval(parse_term) => {
            let term = RcTerm::from_concrete(&parse_term);
            let context = Context::new();
            let (_, inferred) = semantics::infer(&context, &term)?;
            let evaluated = semantics::normalize(&context, &term)?;
            let doc = pretty::pretty_ann(pretty::Options::default(), &evaluated, &inferred);

            println!("{}", doc.pretty(80));
        },
        ReplCommand::TypeOf(parse_term) => {
            let term = RcTerm::from_concrete(&parse_term);
            let context = Context::new();
            let (_, inferred) = semantics::infer(&context, &term)?;
            let doc = inferred.to_doc(pretty::Options::default());

            println!("{}", doc.pretty(80));
        },

        ReplCommand::NoOp => {},
        ReplCommand::Quit => return Err(ReplError::Quit),
    }

    Ok(())
}

enum ReplError {
    Parse(parse::ParseError),
    Type(semantics::TypeError),
    Quit,
}

impl From<parse::ParseError> for ReplError {
    fn from(src: parse::ParseError) -> ReplError {
        ReplError::Parse(src)
    }
}

impl From<semantics::TypeError> for ReplError {
    fn from(src: semantics::TypeError) -> ReplError {
        ReplError::Type(src)
    }
}

impl From<semantics::InternalError> for ReplError {
    fn from(src: semantics::InternalError) -> ReplError {
        ReplError::Type(src.into())
    }
}