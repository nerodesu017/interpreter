use std::io::{BufRead, BufReader, Write};

use crate::{
    error::{ParserErrorTrait, ScannerErrorTrait},
    expr::Expr,
    parser,
    scanner::Scanner,
    token::{Token, TokenType},
    visitor::AstPrinter,
};

pub struct Lox {
    pub had_error: bool,
}

impl ScannerErrorTrait for Lox {
    fn scanner_error(&mut self, line: usize, msg: String) {
        self.report(line, String::from(""), msg);
    }
}

impl ParserErrorTrait for Lox {
    fn parser_error(&mut self, token: Token, message: String) {
        if matches!(token.ttype, TokenType::Eof) {
            self.report(token.line, " at end".to_owned(), message);
        } else {
            self.report(
                token.line,
                " at '".to_owned() + token.lexeme.to_string().as_str() + "'",
                message,
            );
        }
    }
}

impl Lox {
    fn report(&mut self, line: usize, where_: String, msg: String) {
        eprintln!("[line {}] Error{}: {}", line, where_, msg);

        self.had_error = true;
    }

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
        let mut parser: parser::Parser = parser::Parser::new(tokens, self);
        let expression: Expr = match parser.parse() {
            Ok(val) => val,
            Err(_) => return,
        };

        if self.had_error {
            return;
        }

        println!("{}", AstPrinter {}.print(expression))

        // for token in tokens {
        //     println!("{}", token);
        // }
    }
}
