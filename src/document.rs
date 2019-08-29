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

pub fn document_authors(ast: Pair<Rule>) {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            Rule::document_authors_inline_form => println!("da : daif"),
            Rule::document_authors_attribute_form => println!("da: daaf"),
            _ => unreachable!(),
        }
    }
}

// FIXME
pub fn document_attribute_substitution(_ast: Pair<Rule>) {
    println!("doc attr substi");
}
pub fn document_attribute_declaration(ast: Pair<Rule>) {
    for elem in ast.into_inner() {
        match elem.as_rule() {
            Rule::document_attribute_name => println!("dad : dan"),
            Rule::document_attribute_value => println!("dad : dav"),
            _ => unreachable!(),
        }
    }
}

pub fn document_attribute_reset(ast: Pair<Rule>) {
    for elem in ast.into_inner() {
        match elem.as_rule() {
            Rule::document_attribute_name => println!("doc attr reset : doc attr name"),
            _ => unreachable!(),
        }
    }
}

// table_of_contents_macro = { "toc::[]" ~ EOL }
pub fn table_of_contents_macro(_ast: Pair<Rule>) {
    // do nothing currently
}

pub fn front_matter(ast: Pair<Rule>) {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            Rule::yaml_front_matter => yaml_front_matter(e),
            _ => unreachable!(),
        }
    }
}
pub fn yaml_front_matter(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::yaml_front_matter_token => println!("token in yml front matter"), // do nothing
            Rule::yaml_front_matter_content => println!("yaml front matter content"),
            _ => unreachable!(),
        }
    }
}
