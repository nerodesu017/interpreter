use std::io::{BufRead, BufReader};

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
    pub fn run_file(&mut self, str: String) {}

    pub fn run_prompt(&mut self) {
        let input = std::io::stdin();
        let mut reader = BufReader::new(input);

        let mut line = String::new();

        loop {
            print!("> ");
            let result = reader.read_line(&mut line);
            if result.is_err() {
                break;
            }
            self.run(line.clone());
        }
    }

    fn run(&mut self, source: String) {
      let scanner = Scanner::new(source);
      let tokens: Vec<Token> = scanner.scan_tokens();
  
      for _token in tokens {
          // println!("{}", token);
      }
  }
}
