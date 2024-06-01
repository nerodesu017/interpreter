use std::io::{BufRead, BufReader, Write};

use crate::{error::ScannerError, scanner::Scanner, token::Token};

pub struct Lox {
    pub had_error: bool,
}

impl ScannerError for Lox {
    fn error(&mut self, line: usize, msg: String) {
        self.report(line, String::from(""), msg);
    }

    fn report(&mut self, line: usize, where_: String, msg: String) {
        eprintln!("[line {}] Error{}: {}", line, where_, msg);

        self.had_error = true;
    }
}

impl Lox {
    pub fn run_file(&mut self, str: String) {


        if self.had_error {
            std::process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        let input = std::io::stdin();
        let mut output = std::io::stdout();
        let mut reader = BufReader::new(input);

        let mut line: String;

        loop {
            line = String::new();
            print!("> ");
            output.flush().unwrap();

            let result = reader.read_line(&mut line);
            if result.is_err() {
                break;
            }
            line = line.trim_end().to_owned();


            output.flush().unwrap();
            self.run(line.clone());
            self.had_error = false;
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source, self);
        let tokens: Vec<Token> = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token);
        }
    }
}
