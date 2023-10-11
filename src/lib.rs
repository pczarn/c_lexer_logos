use logos::Logos;

use self::token::Token;

pub mod token;
pub mod error;

fn parse(input: &str) -> Result<Vec<Token>, error::Error> {
    let mut lex = Token::lexer(input);
    let mut result = vec![];

    while let Some(maybe_tok) = lex.next() {
        match maybe_tok {
            Ok(tok) => {
                result.push(tok);
            }
            Err(_err) => {
                return Err(error::Error::LexingIncomplete);
            }
        }
    }
    Ok(result)
}

/// Lexer implementation
#[derive(Debug, Copy, Clone)]
pub struct Lexer;

impl Lexer {
    /// Transform string to stream of tokens
    pub fn lex(input: &str) -> Result<Vec<Token>, error::Error> {
        let mut tokens = parse(input)?;
        tokens.push(Token::EOF);
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Number;

    use super::*;

    #[test]
    fn it_works() {
        let result = Lexer::lex("sizeof(123) != 32");
        assert_eq!(result, Ok(vec![Token::SIZEOF, Token::LParen, Token::NumericLiteral(Number::new(123, 0, 0, 10)), Token::RParen, Token::NeOp, Token::NumericLiteral(Number::new(32, 0, 0, 10)), Token::EOF]));
    }
}
