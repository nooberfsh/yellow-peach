use crate::code_gen::gen_ast::gen_grammar;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub mod ast;
pub mod code_gen;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod token;

fn main() {
    println!("Hello, world!");
    //let s = std::fs::read_to_string("grammars/sql.yp").unwrap();
    let s = std::fs::read_to_string("test.yp").unwrap();
    let lexer = Lexer::new(&s);
    let mut parser = Parser::new(lexer).unwrap();
    let grammar = parser.parse_grammar().unwrap();
    println!("{:#?}", grammar);
    let p = gen_grammar(&grammar);
    println!("{}", p);
}
