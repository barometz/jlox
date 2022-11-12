use std::{
    env,
    io::{stdin, Read, Write},
    path::{Path, PathBuf},
};
use thiserror::Error;

mod ast_printer;
mod expr;
mod scanner;
mod token;

#[derive(Error, Debug)]
enum ELoxError {
    #[error("{0:?}")]
    ScannerError(Vec<scanner::ScannerError>),
    #[error(" Failed to read: {0}")]
    FileNotFoundError(std::io::Error),
}

#[derive(Error, Debug)]
#[error("{path}:{error}")]
struct LoxError {
    path: PathBuf,
    error: ELoxError,
}

impl From<Vec<scanner::ScannerError>> for ELoxError {
    fn from(error: Vec<scanner::ScannerError>) -> Self {
        ELoxError::ScannerError(error)
    }
}

impl From<std::io::Error> for ELoxError {
    fn from(error: std::io::Error) -> Self {
        ELoxError::FileNotFoundError(error)
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
            Err(error) => {
                return Err(LoxError {
                    path: path.into(),
                    error: error.into(),
                })
            }
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
            Err(error) => Err(LoxError {
                path,
                error: error.into(),
            }),
        },
        Err(error) => Err(LoxError {
            path,
            error: error.into(),
        }),
    }
}

fn run(path: &Path, source: &str) -> Result<(), LoxError> {
    let mut scanner = scanner::Scanner::new(source);

    match scanner.scan_tokens() {
        Ok(tokens) => {
            for token in &tokens {
                println!("{}", token)
            }
            Ok(())
        }
        Err(errors) => Err(LoxError {
            path: path.into(),
            error: errors.into(),
        }),
    }
}
