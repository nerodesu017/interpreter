mod scanner;
mod token;
mod error;
mod lox;


fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);

    let mut lox = lox::Lox{had_error: false};

    if args.len() > 1 {
        println!("Usage: jlox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        lox.run_file(args[0].to_owned());
    } else {
        lox.run_prompt();
    }
}