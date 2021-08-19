use std::fs;
use std::io;
use std::path::Path;

use std::path::PathBuf;
use structopt::StructOpt;

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

#[derive(Debug, StructOpt)]
#[structopt(name = "yp", about = "An ast generator.")]
struct Opt {
    /// Grammar file path
    #[structopt(parse(from_os_str))]
    grammar_path: PathBuf,

    /// Where to generate files
    #[structopt(parse(from_os_str))]
    #[structopt(short, default_value = ".")]
    out_dir: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let s = std::fs::read_to_string(&opt.grammar_path).expect("read grammar filed failed");
    let lexer = Lexer::new(&s);
    let mut parser = Parser::new(lexer).expect("create parser failed");
    let grammar = parser.parse_grammar().expect("parse grammar failed");
    let mir = mir::lower(&grammar).expect("lower ast to mir failed");
    let cg = CodeGen::new(mir);

    let output = ".";
    create_ast(&cg, output).expect("create ast file failed");
    create_span(&cg, output).expect("create span file failed");
    create_visitor(&cg, output).expect("create visitor file failed");
    create_visitor_mut(&cg, output).expect("create visitor_mut file failed");

    println!("generate success.")
}

fn create_ast<P: AsRef<Path>>(cg: &CodeGen, p: P) -> io::Result<()> {
    let d = cg.gen_ast();
    let mut p = p.as_ref().to_path_buf();
    p.push("ast.rs");
    fs::write(p, &d)
}

fn create_span<P: AsRef<Path>>(cg: &CodeGen, p: P) -> io::Result<()> {
    let d = cg.gen_span();
    let mut p = p.as_ref().to_path_buf();
    p.push("span.rs");
    fs::write(p, &d)
}

fn create_visitor<P: AsRef<Path>>(cg: &CodeGen, p: P) -> io::Result<()> {
    let d = cg.gen_visit(false);
    let mut p = p.as_ref().to_path_buf();
    p.push("visitor.rs");
    fs::write(p, &d)
}

fn create_visitor_mut<P: AsRef<Path>>(cg: &CodeGen, p: P) -> io::Result<()> {
    let d = cg.gen_visit(true);
    let mut p = p.as_ref().to_path_buf();
    p.push("visitor_mut.rs");
    fs::write(p, &d)
}
