#[macro_use]
extern crate pest_derive;

pub mod parse;
use parse::*;

pub mod tpl;
use tpl::*;

pub mod structs;

use chrono::DateTime;
use pest::iterators::Pair;
use pest::Parser;
use std::env;
use std::fs::File;
use std::io::Read;
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
    let m = File::open(Path::new(path.as_str()))
        .or(Err(-1))?
        .metadata()
        .or(Err(-1))
        .unwrap()
        .modified()
        .or(Err(-1))
        .unwrap();

    let m: DateTime<chrono::offset::Local> = DateTime::from(m);

    let after = precompile(buffer);

    convert(after.as_str(), m.format("%+").to_string().as_str());

    // add toc to str will destroy the ast

    Ok(())
}

/*
block_delimiter = {
    literal_block_delimiter ....
    | fenced_block_delimiter
    | listing_block_delimiter ----
    | example_block_delimiter ====
    | comment_block_delimiter ////
    | quote_block_delimiter ____
    | sidebar_block_delimiter ****
}
*/
enum mode {
    Normal,
    Example,
    Quote,
    Literal,
    Listing,
    Sidebar,
    Comment,
}
use std::collections::HashMap;

// FIXME
// should not fix quote/example block
// which is in literal or source block
pub fn precompile(before: String) -> String {
    let mut lines: Vec<&str> = before.split("\n").collect();

    let mut line_to_mode: HashMap<char, mode> = vec![
        ('=', mode::Example),
        ('.', mode::Literal),
        ('_', mode::Quote),
        ('-', mode::Listing),
        ('*', mode::Sidebar),
        ('/', mode::Comment),
    ]
    .into_iter()
    .collect();

    let mut final_lines = vec![];
    let mut mark_stack = vec![];
    let mut mode_stack = vec![&mode::Normal];
    lines.iter().for_each(|&l| {
        // if match key of mode, then:
        // find any same line in stack
        // if there is same line in stack
        //    pop until match
        // else
        //    push to stack
        //    change mode
        let line = l.trim();
        //        let c0 = line.chars().nth(0).unwrap();
        let c0 = if line.len() > 0 {
            line.chars().nth(0).unwrap()
        } else {
            '#'
        };
        for (idx, c) in line.chars().enumerate() {
            if !line_to_mode.contains_key(&c) || c != c0 {
                break;
            }

            // final character
            if idx == line.len() - 1 {
                // only :
                // 1. sidebar; 2. example; 3. quote
                // support nested block
                match mode_stack.last().unwrap(){
                    mode::Sidebar => {}
                    mode::Example => {}
                    mode::Quote => {}
                    _ => break,
                }
                // find the match from stack top to stack bottom
                if mark_stack.contains(&line) {
                    // if there is a match, pop && push to final lines until match
                    loop {
                        let mark = mark_stack.pop().unwrap();
                        if mark == line {
                            break;
                        }
                        // mark != line
                        // should append this to out document to satisfy
                        // the parser situation
                        final_lines.push(mark);
                    }
                } else {
                    // if there is no match, push this line to stack
                    mark_stack.push(line);
                    mode_stack.push(line_to_mode.get(&c0).unwrap())
                }
            }
        }
        final_lines.push(l);
    });

    final_lines.join("\n")
}

pub fn convert(query: &str, modify_time: &str) {
    let parse_result = ExprParser::parse(Rule::pre_flight_document, query);
    match parse_result {
        Ok(top_ast) => {
            let result = walk_tree(top_ast.clone().next().unwrap());
            //println!("{:#?}", top_ast);
            //            println!("the parse result is \n{}", result);
            println!(
                "{}",
                page_tpl()
                    .replace("{{content}}", result.as_str())
                    .replace("{{header}}", "")
                    .replace(
                        "{{footer}}",
                        format!(
                            r#"<div id="footer-text">Last updated {}</div>"#,
                            modify_time
                        )
                        .as_str(),
                    )
            );
        }
        Err(e) => {
            dbg!(e);
        }
    }
}

pub fn walk_tree(ast: Pair<Rule>) -> String {
    //println!("top match");
    let mut result = String::new();
    match ast.as_rule() {
        Rule::pre_flight_document => result += pre_flight_document(ast).as_str(),
        _ => unreachable!(),
    }
    result
}
