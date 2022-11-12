use std::{fs::File, io::Write, path::PathBuf, process::ExitCode};

static EXPRESSION_GRAMMAR: &[&str] = &[
    // "Expr     : Binary | Grouping | Literal | Unary",
    "Binary   : lhs: Expr<'s>, operator: Token<'s>, rhs: Expr<'s>",
    "Grouping : expression: Expr<'s>",
    "Literal  : value: Literal",
    "Unary    : operator: Token<'s>, operand: Expr<'s>",
];

struct Symbol {
    name: String,
    symbol_type: String,
}

struct Rule {
    head: String,
    body: Vec<Symbol>,
}

fn parse_grammar(input: &[&str]) -> Vec<Rule> {
    let mut result = Vec::<Rule>::new();
    for rule in input {
        let (head, body) = rule.split_once(':').unwrap();
        result.push(Rule {
            head: head.trim().into(),
            body: body
                .split(',')
                .map(|s| {
                    let (name, symbol_type) = s.split_once(':').unwrap();
                    Symbol {
                        name: name.trim().into(),
                        symbol_type: symbol_type.trim().into(),
                    }
                })
                .collect(),
        });
    }
    result
}

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

    let grammar = parse_grammar(EXPRESSION_GRAMMAR);
    match file {
        Ok(mut file) => match writeln!(file, "use crate::token::{{Literal, Token}};")
            .and_then(|_| define_ast(&mut file, &grammar))
            .and_then(|_| define_accepter(&mut file, &grammar))
            .and_then(|_| define_visitor(&mut file, &grammar))
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

fn define_ast(out: &mut dyn Write, grammar: &[Rule]) -> Result<(), std::io::Error> {
    writeln!(out, "pub enum Expr<'s> {{")?;

    for rule in grammar {
        writeln!(out, "    {} {{", rule.head)?;
        for symbol in &rule.body {
            writeln!(out, "        {}: Box<{}>,", symbol.name, symbol.symbol_type)?;
        }
        writeln!(out, "    }},")?;
    }

    writeln!(out, "}}")?;
    Ok(())
}

fn define_accepter(out: &mut dyn Write, grammar: &[Rule]) -> Result<(), std::io::Error> {
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
        let match_fields = rule
            .body
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<&str>>()
            .join(", ");

        writeln!(
            out,
            "            Expr::{} {{ {} }} => visitor.visit_{}({}),",
            rule.head,
            match_fields,
            rule.head.to_ascii_lowercase(),
            match_fields,
        )?;
    }

    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    Ok(())
}

fn define_visitor(out: &mut dyn Write, grammar: &[Rule]) -> Result<(), std::io::Error> {
    writeln!(out, "pub trait ExprVisitor<'s, R> {{")?;

    for rule in grammar {
        write!(
            out,
            "    fn visit_{}(&mut self",
            rule.head.to_ascii_lowercase()
        )?;
        for symbol in &rule.body {
            write!(out, ", {}: &{}", symbol.name, symbol.symbol_type)?;
        }
        writeln!(out, ") -> R;")?;
    }

    writeln!(out, "}}")?;
    Ok(())
}
