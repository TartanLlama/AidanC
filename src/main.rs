#![feature(io)]

#[macro_use] extern crate itertools;
use itertools::Itertools;
use itertools::PutBackN;

extern crate aidan_c;

use aidan_c::lex;
use aidan_c::lex::tokens::Token;

use std::fs::File;
use std::io::Read;

fn main() {
    let f = File::open("/home/simon/play/AidanC/samples/factorial.adn").unwrap();
    let mut chars = PutBackN::new(f.chars());

    loop {
        match lex::lex(&mut chars).unwrap() {
            Token::EOF => {println!("EOF");return ()},
            other =>         println!("{:?}", other),
        }
    }
}


