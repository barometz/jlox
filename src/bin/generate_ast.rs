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
        Ok(mut file) => match writeln!(file, "use crate::token::{{Literal, Token}};")
            .and_then(|_| define_ast(&mut file, EXPRESSION_GRAMMAR))
            .and_then(|_| define_accepter(&mut file, EXPRESSION_GRAMMAR))
            .and_then(|_| define_visitor(&mut file, EXPRESSION_GRAMMAR))
        {
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
    writeln!(out, "pub enum Expr<'s> {{")?;

    for rule in grammar {
        let (symbol, expression) = rule.split_once(':').unwrap();
        writeln!(out, "    {} {{", symbol.trim())?;
        let fields = expression.split(',');
        for field in fields {
            let (label, field_type) = field.split_once(':').unwrap();
            writeln!(out, "        {}: Box<{}>,", label.trim(), field_type.trim())?;
        }
        writeln!(out, "    }},")?;
    }

    writeln!(out, "}}")?;
    Ok(())
}

fn define_accepter(out: &mut dyn Write, grammar: &[&str]) -> Result<(), std::io::Error> {
    // Not sure it makes a lot of sense to call this a visitor pattern - it
    // certainly isn't what Crafting Interpreters or Design Patterns describe,
    // and it doesn't match the Rust Design Patterns description either.
    // Nevertheless, this seems like a useful way to go about it.

    writeln!(out, "impl<'s> Expr<'s> {{")?;
    writeln!(
        out,
        "    pub fn accept<R>(&self, visitor: &mut dyn ExprVisitor<'s, R>) -> R {{"
    )?;
    writeln!(out, "        match self {{")?;

    for rule in grammar {
        let (symbol, expression) = rule.split_once(':').unwrap();
        let fields = expression.split(',');

        let match_fields = fields
            .map(|f| f.split_once(':').unwrap().0.trim())
            .collect::<Vec<&str>>()
            .join(", ");

        writeln!(
            out,
            "            Expr::{} {{ {} }} => visitor.visit_{}({}),",
            symbol.trim(),
            match_fields,
            symbol.trim().to_ascii_lowercase(),
            match_fields,
        )?;
    }

    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    Ok(())
}

fn define_visitor(out: &mut dyn Write, grammar: &[&str]) -> Result<(), std::io::Error> {
    writeln!(out, "pub trait ExprVisitor<'s, R> {{")?;

    // lots of code duplication here, but let's write everything once before
    // trying to factor out the generic bits
    for rule in grammar {
        let (symbol, expression) = rule.split_once(':').unwrap();
        let fields = expression.split(',');
        write!(
            out,
            "    fn visit_{}(&mut self",
            symbol.trim().to_ascii_lowercase()
        )?;
        for field in fields {
            let (label, field_type) = field.split_once(':').unwrap();
            write!(out, ", {}: &{}", label.trim(), field_type.trim())?;
        }
        writeln!(out, ") -> R;")?;
    }

    writeln!(out, "}}")?;
    Ok(())
}
