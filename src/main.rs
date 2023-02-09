use calculator::eval;
use calculator::CalculatorError;

fn main() {
    loop {
        match repl() {
            Ok(_) => {}
            Err(e) => eprintln!("{e:?}"),
        }
    }
}

fn repl() -> Result<(), CalculatorError> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    println!("{:?}", eval(input)?);

    Ok(())
}
