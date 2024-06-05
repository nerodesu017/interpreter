use crate::token::Token;

pub trait ScannerErrorTrait {
    fn scanner_error(&mut self, line: usize, msg: String);
}

pub trait ParserErrorTrait {
    fn parser_error(&mut self, token: Token, message: String);
}

// pub enum ParserError {
//     ExpectExpression,
//     ExpectLeftParenAfterExpression,
//     Custom(String)
// }

// impl ToString for ParserError {
//     fn to_string(&self) -> String {
//         match self {
//             ParserError::ExpectExpression => "Expect expression.",
//             ParserError::ExpectLeftParenAfterExpression => "Expect ')' after expression.",
//             ParserError::Custom(err) => err
//         }.to_owned()
//     }
// }

#[derive(Debug)]
pub struct ParserError {}