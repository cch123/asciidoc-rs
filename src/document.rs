use crate::parse::*;
use crate::section::*;
use pest::iterators::Pair;
pub fn document_blocks(ast: Pair<Rule>) -> String {
    let mut result = String::new();

    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::document_block => {
                result += document_block(e).as_str();
            }
            _ => unreachable!(),
        }
    }
    result
}

pub fn document_block(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::document_header => {
                result += document_header(e).as_str();
            }
            Rule::preamble => {
                result += preamble(e).as_str();
            }
            Rule::sections => {
                // TODO, merge sections
                // build nested section tree
                let b = sections(e);
                //println!("{:?}", b);
                result += b.as_str();
            }
            _ => unreachable!(),
        }
    }
    result
}

pub fn document_header(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::title_elements => {
                result = format!("{}{}{}{}\n", result, "<h1>", e.as_str(), "</h1>");
            }
            Rule::inline_element_id => {
                inline_element_id(e);
            }
            Rule::document_authors => {
                document_authors(e);
            }
            Rule::document_revision => {
                document_revision(e);
            }
            _ => unreachable!(),
        }
    }
    result
}

pub fn document_revision(ast: Pair<Rule>) {
    for elem in ast.into_inner() {
        match elem.as_rule() {
            Rule::document_revision_number => println!("dv : drn"),
            Rule::document_revision_date => println!("dv : drd"),
            Rule::document_revision_remark => println!("dv : drr"),
            _ => unreachable!(),
        }
    }
}

pub fn pre_flight_document(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    let c = ast.into_inner();
    for elem in c {
        match elem.as_rule() {
            //~ front_matter*
            Rule::front_matter => front_matter(elem),
            //~ document_block
            Rule::document_blocks => result += document_blocks(elem).as_str(),
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
    result
}
