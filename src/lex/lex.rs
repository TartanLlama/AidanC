use std::io;
use lex::tokens::Token;

fn lex <Stream: io::Read> (input: &Stream) -> Result<Token, String> {
    Ok(Token::Var)
}
