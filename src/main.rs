#![allow(clippy::result_large_err)]

use std::{
    env,
    io::{stdin, Read, Write},
    path::{Path, PathBuf},
};
use thiserror::Error;

use jlox::{ast_printer, parser, scanner};

#[derive(Error, Debug)]
enum ELoxError {
    #[error("{0:?}")]
    Scanner(Vec<scanner::ScannerError>),
    #[error("{0:}")]
    Parser(parser::ParserError),
    #[error(" Failed to read: {0}")]
    FileNotFound(std::io::Error),
}

#[derive(Error, Debug)]
#[error("{path}:{error}")]
struct LoxError {
    path: PathBuf,
    error: ELoxError,
}

impl LoxError {
    fn new<T>(path: &Path, error: T) -> LoxError
    where
        ELoxError: From<T>,
    {
        LoxError {
            path: path.to_owned(),
            error: error.into(),
        }
    }
}

impl From<Vec<scanner::ScannerError>> for ELoxError {
    fn from(error: Vec<scanner::ScannerError>) -> Self {
        ELoxError::Scanner(error)
    }
}

impl From<parser::ParserError> for ELoxError {
    fn from(error: parser::ParserError) -> Self {
        ELoxError::Parser(error)
    }
}

impl From<std::io::Error> for ELoxError {
    fn from(error: std::io::Error) -> Self {
        ELoxError::FileNotFound(error)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: jlox [script]");
            Ok(())
        }
    };

    if let Err(error) = result {
        eprintln!("{}", error)
    }
}

fn run_prompt() -> Result<(), LoxError> {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        let path = Path::new("<stdin>");

        match stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => match run(path, &line) {
                Ok(_) => continue,
                Err(error) => eprintln!("{}", error),
            },
            Err(error) => return Err(LoxError::new(path, error)),
        }
    }

    Ok(())
}

fn run_file(path: &str) -> Result<(), LoxError> {
    let path: std::path::PathBuf = path.into();

    let mut source = String::new();

    match std::fs::File::open(&path) {
        Ok(mut file) => match file.read_to_string(&mut source) {
            Ok(_) => run(&path, &source),
            Err(error) => Err(LoxError::new(&path, error)),
        },
        Err(error) => Err(LoxError::new(&path, error)),
    }
}

fn run(path: &Path, source: &str) -> Result<(), LoxError> {
    let mut scanner = scanner::Scanner::new(source);

    match scanner.scan_tokens() {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(&tokens);
            match parser.parse() {
                Ok(expr) => {
                    // TODO: add non-mutable visitor trait
                    let mut printer = ast_printer::AstPrinter {};
                    println!("{}", printer.print(&expr));
                    Ok(())
                }
                Err(error) => Err(LoxError::new(path, error)),
            }
        }
        Err(errors) => Err(LoxError::new(path, errors)),
    }
}
