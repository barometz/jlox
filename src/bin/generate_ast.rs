use std::{fs::File, io::Write, path::PathBuf, process::ExitCode};

// TODO: it would be nice if not everything was boxed in the Expr enum
static EXPRESSION_GRAMMAR: &[&str] = &[
    // "Expr     : Binary | Grouping | Literal | Unary",
    "Binary   : lhs: Expr, operator: Token, rhs: Expr",
    // Is generically supporting different kinds of ternary operators overkill?
    // Yes. Having acknowledged that: how often do you get the chance to talk
    // about a middle-hand side and a left-hand operator?
    "Ternary  : lhs: Expr, lho: Token, mhs: Expr, rho: Token, rhs: Expr",
    "Grouping : expression: Expr",
    "Literal  : value: Literal",
    "Unary    : operator: Token, operand: Expr",
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

    let header = r#"// generated by: cargo run --bin generate_ast src

use crate::token::{Literal, Token};

"#;

    let grammar = parse_grammar(EXPRESSION_GRAMMAR);
    match file {
        Ok(mut file) => match write!(file, "{}", header)
            .and_then(|_| define_ast(&mut file, &grammar))
            .and_then(|_| define_impl(&mut file, &grammar))
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
    writeln!(out, "#[derive(Debug, PartialEq)]")?;
    writeln!(out, "pub enum Expr {{")?;

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

fn define_impl(out: &mut dyn Write, grammar: &[Rule]) -> Result<(), std::io::Error> {
    writeln!(out, "impl Expr {{")?;
    define_accepter(out, grammar)?;
    for rule in grammar {
        define_new(out, rule)?;
    }
    writeln!(out, "}}")?;
    Ok(())
}

fn define_accepter(out: &mut dyn Write, grammar: &[Rule]) -> Result<(), std::io::Error> {
    // Not sure it makes a lot of sense to call this a visitor pattern - it
    // certainly isn't what Crafting Interpreters or Design Patterns describe,
    // and it doesn't match the Rust Design Patterns description either.
    // Nevertheless, this seems like a useful way to go about it.

    writeln!(
        out,
        "    pub fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {{"
    )?;
    writeln!(out, "        match self {{")?;

    for rule in grammar {
        let match_fields = rule
            .body
            .iter()
            .map(|s| s.name.clone())
            .collect::<Vec<String>>()
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
    Ok(())
}

fn define_new(out: &mut dyn Write, rule: &Rule) -> Result<(), std::io::Error> {
    writeln!(
        out,
        "    pub fn new_{}({}) -> Expr {{",
        rule.head.to_ascii_lowercase(),
        rule.body
            .iter()
            .map(|sym| format!("{}: {}", sym.name, sym.symbol_type))
            .collect::<Vec<String>>()
            .join(", ")
    )?;
    writeln!(
        out,
        "        Expr::{} {{ {} }}",
        rule.head,
        rule.body
            .iter()
            .map(|sym| format!("{0}: Box::new({0})", sym.name))
            .collect::<Vec<String>>()
            .join(", ")
    )?;

    writeln!(out, "    }}")?;
    Ok(())
}

fn define_visitor(out: &mut dyn Write, grammar: &[Rule]) -> Result<(), std::io::Error> {
    writeln!(out, "pub trait ExprVisitor<R> {{")?;

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
