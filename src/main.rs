use calculator::eval;
use calculator::CalculatorError;

use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        match repl() {
            Ok(_) => {}
            Err(e) => eprintln!("{e:?}"),
        }
    }
}

fn repl() -> Result<(), CalculatorError> {
    // reset colors
    stdout().write_all(b"\x1b[0m")?;
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let y = eval(input)?;
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
