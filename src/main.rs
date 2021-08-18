use crate::code_gen::CodeGen;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub mod ast;
pub mod code_gen;
pub mod lexer;
pub mod mir;
pub mod parser;
pub mod span;
pub mod token;
pub mod util;
pub mod visit;

fn main() {
    println!("Hello, world!");
    let s = std::fs::read_to_string("grammars/sql.yp").unwrap();
    //let s = std::fs::read_to_string("test.yp").unwrap();
    let lexer = Lexer::new(&s);
    let mut parser = Parser::new(lexer).unwrap();
    let grammar = parser.parse_grammar().unwrap();
    let mir = mir::lower(&grammar).unwrap();
    let cg = CodeGen::new(mir);
    let p = cg.gen_ast();
    println!("{}", p);
}
