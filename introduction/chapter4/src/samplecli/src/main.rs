// RPN Calculator with error handling using anyhow
use anyhow::{bail, ensure, Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;

struct RpnCalculator(bool);

#[derive(Parser, Debug)]
#[command(
    name = "My RPN program",
    version = "1.0.0",
    author = "N.N.",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    #[arg(short, long)]
    verbose: bool,

    #[arg(value_name = "FILE")]
    formula_file: Option<PathBuf>,
}

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack
                    .pop()
                    .with_context(|| format!("invalid syntax at {}", pos))?;
                let x = stack
                    .pop()
                    .with_context(|| format!("invalid syntax at {}", pos))?;

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("Invalid token at {}", pos),
                };
                stack.push(res);
            }

            if self.0 {
                println!("token: {:?} and stack: {:?}", tokens, stack);
            }
        }

        ensure!(
            stack.len() == 1,
            "invalid syntax: remaining stack {:?}",
            stack
        );

        Ok(stack[0])
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.context("Failed to read line from input")?;

        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("Evaluation error: {}", e),
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(&path)
            .with_context(|| format!("Could not open file: {}", path.display()))?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)?;
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
    }

    #[test]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        assert!(calc.eval("1 1 ^").is_err());
        assert!(calc.eval("1 +").is_err());
    }
}
