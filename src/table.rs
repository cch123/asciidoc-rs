use crate::parse::*;
use pest::iterators::Pair;

pub fn table(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::element_attributes => {
                element_attributes(e);
            }
            Rule::table_delimiter => {} // do nothing
            Rule::table_line_header => table_line_header(e),
            Rule::table_line => table_line(e),
            _ => unreachable!(),
        }
    }
}
// table_line = { !table_delimiter ~ table_cell+ ~ EOL ~ blank_line* }
pub fn table_line(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::table_cell => table_cell(e),
            Rule::blank_line => blank_line(e),
            _ => unreachable!(),
        }
    }
}

pub fn table_line_header(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::table_cell => table_cell(e),
            Rule::blank_line => blank_line(e),
            _ => unreachable!(),
        }
    }
}

pub fn table_cell(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::inline_element => {
                inline_element(e);
            }
            _ => unreachable!(),
        }
    }
}
