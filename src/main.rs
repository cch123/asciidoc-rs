#![recursion_limit = "1024"]

extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate serde_json;
use pest::prec_climber::{Assoc, Operator, PrecClimber};

//use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ExprParser;

/// error occurred when parsing user input
#[derive(Debug)]
pub struct ParseError {
    pub location: pest::error::InputLocation,
    pub expected: String,
}

pub fn convert(
    query: String
) {
    let x = query.as_str();
    let parse_result = ExprParser::parse(Rule::pre_flight_document, x);
    println!("{:#?}", parse_result);
}

use pest::iterators::Pair;

fn main() {
    let str = r#"= title
== second title
content line
[source, c]
----
int main() {
}
----
"#.to_string();
    convert(str);
}