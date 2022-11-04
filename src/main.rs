use std::{
    env,
    io::{stderr, stdin, Read, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            writeln!(stderr(), "Usage: jlox [script]").unwrap();
        }
    };
}

fn run_prompt() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => run(&line),
            Err(_) => break,
        }
    }
}

fn run_file(path: &str) {
    let path: std::path::PathBuf = path.into();

    let mut source = String::new();
    if let Err(error) = std::fs::File::open(&path)
        .and_then(|mut file| file.read_to_string(&mut source))
        .map(|_| run(&source))
    {
        writeln!(
            stderr(),
            "Failed to read source file @ {}: {}",
            path.display(),
            error
        )
        .unwrap();
    }
}

fn run(source: &str) {}
