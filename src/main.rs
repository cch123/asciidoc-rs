extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parse;
use parse::*;

use pest::iterators::Pair;
use pest::{Parser, RuleType};
use std::env;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::path::Path;

fn main() -> Result<(), i32> {
    if env::args().len() < 2 {
        println!("Please input file name!");
        return Ok(());
    }

    let path = env::args().nth(1).ok_or(-1)?;
    let mut buffer = String::new();
    File::open(Path::new(path.as_str()))
        .or(Err(-1))?
        .read_to_string(&mut buffer)
        .or(Err(-1))?;

    convert(buffer.as_str());

    // add toc to str will destroy the ast

    return Ok(());
}

pub fn convert(query: &str) {
    let parse_result = ExprParser::parse(Rule::pre_flight_document, query);
    match parse_result {
        Ok(mut top_ast) => {
            walk_tree(top_ast.clone().next().unwrap());
            println!("{:#?}", top_ast);
            //println!("{}", format_pair(top_ast.next().unwrap(), 0, true));
        }
        Err(e) => {
            dbg!(e);
        }
    }
}

pub fn walk_tree(ast: Pair<Rule>) {
    println!("top match");
    match ast.as_rule() {
        Rule::pre_flight_document => pre_flight_document(ast),
        _ => unreachable!(),
    }
}
