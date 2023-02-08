use calculator::eval;

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input);

        match eval(input) {
            Ok(n) => println!("{n}"),
            Err(e) => eprintln!("{e:?}"),
        }
    }
}
