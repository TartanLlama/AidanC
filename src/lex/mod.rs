pub mod tokens;

use std::io;
use std::iter::Peekable;
use std::io::Chars;
use lex::tokens::Token;

use itertools::PutBackN;
use itertools::PutBack;


fn eat_whitespace  <R: io::Read> (input: &mut PutBackN<Chars<R>>) {
    loop {
        match input.next() {
            Some (c) => {
                match c {
                    Ok(c) => if !c.is_whitespace() {
                        input.put_back(Ok(c)); return;
                    },
                    _      => { input.put_back(c); return; },
                }
            },
            None      => (),
        }
    }
}

fn checked_lex <R: io::Read> (input: &mut PutBackN<Chars<R>>, string: &str, tok: Token) -> Option<Token> {
    let mut str_chars = string.chars();
    str_chars.next();

    let mut stack = Vec::new();
    
    for sc in str_chars {
        let l = input.next().unwrap().unwrap();

        if sc == l {
            stack.push(sc);
        }
        else {
            input.put_back(Ok(l));
            while !stack.is_empty() {
                input.put_back(Ok(stack.pop().unwrap()));
            }
            return None;
        }
    }

    Some(tok)
}

fn lex_unknown <R: io::Read> (input: &mut PutBackN<Chars<R>>, start: char) -> Token {
    let mut id = String::new();
    id.push(start);

    let lexing_int = start.is_numeric();
    
    loop {
        let ch = input.next().unwrap().unwrap();
        if !ch.is_alphanumeric() {
            input.put_back(Ok(ch));

            if lexing_int {
                return Token::Int (id.parse::<i32>().expect("IDs must not start with a digit"))
            }
                
            return Token::Id (id);
        }
        else {
            id.push(ch);
        }
    }
}


macro_rules! complex_rule {
    ( $start:expr, $input:expr, $($name:expr => $token:expr),* ) => {
        (|| {
            $( match checked_lex(&mut $input, $name, $token) {
                Some(tok) => return tok,
                None      => (),
            };)*
                lex_unknown(&mut $input, $start)
        })()
    };
}

pub fn lex <R: io::Read> (mut input: &mut PutBackN<Chars<R>>) -> Result<Token, String> {
    eat_whitespace(input);

    Ok(
        match input.next() {
            None    => Token::EOF,
            Some(c) => match c.unwrap() {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Mul,
                '/' => Token::Div,
                '(' => Token::LParen,
                ')' => Token::RParen,

                'a' => complex_rule!('a', input, "and" => Token::And),
                'f' => complex_rule!('f', input, "function" => Token::Function),
                'e' => complex_rule!('e', input, "end" => Token::End,
                                     "else" =>Token::Else),
                
                'r' => complex_rule!('r', input, "return" => Token::Return),
                'w' => complex_rule!('w', input, "while" => Token::While),
                'o' => complex_rule!('o', input, "or" => Token::Or),
                'n' => complex_rule!('n', input, "not" => Token::Not),
                'i' => complex_rule!('i', input, "if" => Token::If),

                c   => lex_unknown(input, c),
            },
        }
    )
}

