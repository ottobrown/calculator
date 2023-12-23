use calculator::eval;
use calculator::CalculatorError;

use std::env::args;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut args = args();
    let _ = args.next();

    let s: String = args.collect();

    if !s.is_empty() {
        match eval_and_print(s) {
            Ok(_) => {}
            Err(e) => eprintln!("{e:?}"),
        };
    } else {
        loop {
            match repl() {
                Ok(_) => {}
                Err(e) => eprintln!("{e:?}"),
            }
        }
    }
}

fn eval_and_print(s: String) -> Result<(), CalculatorError> {
    let y = eval(s)?;
    if !y.is_finite() {
        // change color to green
        stdout().write_all(b"\x1b[0;31m")?;
    } else {
        // change color to red
        stdout().write_all(b"\x1b[0;32m")?;
    }

    println!("\t {y}");

    Ok(())
}

fn repl() -> Result<(), CalculatorError> {
    // reset colors
    stdout().write_all(b"\x1b[0m")?;
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    eval_and_print(input)?;

    Ok(())
}
