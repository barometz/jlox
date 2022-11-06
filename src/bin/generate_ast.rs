use std::{fs::File, io::Write, path::PathBuf, process::ExitCode};

static EXPRESSION_GRAMMAR: &[&str] = &[
    // "Expr     : Binary | Grouping | Literal | Unary",
    "Binary   : lhs: Expr<'s>, operator: Token<'s>, rhs: Expr<'s>",
    "Grouping : expression: Expr<'s>",
    "Literal  : value: Literal",
    "Unary    : operator: Token<'s>, operand: Expr<'s>",
];

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: generate_ast <dir>");
        return ExitCode::FAILURE;
    }

    let ast_path: PathBuf = [&args[1], "expr.rs"].iter().collect();
    let file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&ast_path);

    match file {
        Ok(mut file) => match define_ast(&mut file, EXPRESSION_GRAMMAR) {
            Ok(_) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("Failed to write to {}: {}", ast_path.display(), error);
                ExitCode::FAILURE
            }
        },
        Err(error) => {
            eprintln!(
                "Failed to open {} for writing: {}",
                ast_path.display(),
                error
            );
            ExitCode::FAILURE
        }
    }
}

fn define_ast(out: &mut dyn Write, grammar: &[&str]) -> Result<(), std::io::Error> {
    writeln!(out, "use crate::token::{{Literal, Token}};")?;
    writeln!(out, "enum Expr<'s> {{")?;

    for rule in grammar {
        let (class_name, fields) = rule.split_once(':').unwrap();
        writeln!(out, "    {} {{", class_name.trim())?;
        let fields = fields.split(',');
        for field in fields {
            let (label, field_type) = field.split_once(':').unwrap();
            writeln!(out, "        {}: Box<{}>,", label.trim(), field_type.trim())?;
        }
        writeln!(out, "    }},")?;
    }

    writeln!(out, "}}")?;
    Ok(())
}
