#![recursion_limit = "1024"]
pub mod ast;

extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate serde_json;
//use pest::prec_climber::{Assoc, Operator, PrecClimber};

//use pest::error::Error;
use pest::iterators::Pair;
use pest::{Parser, RuleType};
use std::hint::unreachable_unchecked;
use std::num::ParseIntError;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ExprParser;

/// error occurred when parsing user input
#[derive(Debug)]
pub struct ParseError {
    pub location: pest::error::InputLocation,
    pub expected: String,
}

pub fn convert(query: &str) {
    let parse_result = ExprParser::parse(Rule::pre_flight_document, query);
    match parse_result {
        Ok(mut top_ast) => {
            //walk_tree(top_ast.next().unwrap());
            dbg!(top_ast);
        },
        Err(e) => {
            dbg!(e);
        },
    }
}

pub fn document_blocks(ast: Pair<Rule>) {
    println!("doc blocks {:?} ", ast.as_str());
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::document_header => {
                println!("doc header : {:?}", e.as_str());
                document_header(e);
            }
            Rule::sections => {
                sections(e);
            }
            Rule::preamble => {
                println!("preamble start");
                preamble(e);
                println!("preamble end");
            },
            _ => println!("skip in document block"),
        }
    }
}

pub fn preamble(ast: Pair<Rule>) {
    println!("preamble {:?}", ast.as_str());
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::sections => sections(e),
            _ => unreachable!(),
        }
    }
}

// element_attributes = { element_attribute+ }
pub fn element_attributes(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attribute => element_attribute(e),
            _ => unreachable!(),
        }
    }
}

/*
element_attribute = {
    &("[" | "." | "#") // skip if the content does not start with one of those characters
    ~ (
        element_id
        | element_title
        | element_role
        | literal_attribute
        | source_attributes
        | quote_attributes
        | verse_attributes
        | admonition_marker_attribute
        | horizontal_layout
        | attribute_group
    )
}
*/
pub fn element_attribute(ast: Pair<Rule>) {
    let elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::element_id => println!("elem id"),
        Rule::element_title => println!("elem title"),
        Rule::element_role => println!("elem rol"),
        Rule::literal_attribute => println!("lit attr"),
        Rule::source_attributes => println!("src attr"),
        Rule::quote_attributes => println!("quo attr"),
        Rule::verse_attributes => println!("verse attr"),
        Rule::admonition_marker_attribute => println!("adm m attr"),
        Rule::horizontal_layout => println!("ho la"),
        Rule::attribute_group => println!("attr group"),
        _ => unreachable!(),
    }
}

/*
first_paragraph_line = @{
    !(labeled_list_item_term ~ labeled_list_item_separator)
    ~ simple_word ~ inline_element* ~ line_break? ~ EOL
}
*/
pub fn first_paragraph_line(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::simple_word => println!("simple word"),
            Rule::inline_element => println!("inline ele 1"),
            Rule::line_break => println!("line break"),
            _ => unreachable!(),
        }
    }
}

/*
inline_elements = @{
        single_line_comment
        | (!block_delimiter ~ inline_element+ ~ line_break? ~ EOL)
}
*/
pub fn inline_elements(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::single_line_comment => println!("single_line cmt"),
            Rule::inline_element => println!("inline ele"),
            _ => unreachable!(),
        }
    }
}

pub fn other_paragraph_line(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::inline_elements => inline_elements(e),
            _ => unreachable!(),
        }
    }
}

pub fn simple_paragraph(ast: Pair<Rule>) {
    let mut c = ast.into_inner();
    for elem in c {
        match elem.as_rule() {
            Rule::element_attributes => element_attributes(elem),
            Rule::first_paragraph_line => first_paragraph_line(elem),
            Rule::other_paragraph_line => other_paragraph_line(elem),
            _ => unreachable!(),
        }
    }
}

/*
document_header = {
    "=" ~ WS+ ~ title_elements ~ inline_element_id* ~ EOL
    ~ document_authors?
    ~ document_revision?
}
*/
pub fn document_header(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for elem in elems {
        match elem.as_rule() {
            Rule::title_elements => title_elements(elem),
            Rule::inline_element_id => inline_element_id(elem),
            Rule::document_authors => document_authors(elem),
            Rule::document_revision => document_revision(elem),
            _ => unreachable!(),
        }
    }
}

/*
document_revision = {
    WS* ~ !(":"|"=") ~
    ((document_revision_number ~ ","? ~ document_revision_date? ~ ":"? ~ document_revision_remark?)
    | (document_revision_date ~ ":"? ~ document_revision_remark?))
    ~ EOL
}
*/
pub fn document_revision(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for elem in elems {
        match elem.as_rule() {
            Rule::document_revision_number => println!("dv : drn"),
            Rule::document_revision_date => println!("dv : drd"),
            Rule::document_revision_remark => println!("dv : drr"),
            _ => unreachable!(),
        }
    }
}

/*
document_authors = {
    WS* ~ !"=" ~
    (document_authors_inline_form
    | document_authors_attribute_form)
}
*/
pub fn document_authors(ast: Pair<Rule>) {
    let elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::document_authors_inline_form => println!("da : daif"),
        Rule::document_authors_attribute_form => println!("da: daaf"),
        _ => unreachable!(),
    }
}

/*
inline_element_id = { "[[" ~ ID ~ "]]" ~ WS* }
*/
pub fn inline_element_id(ast: Pair<Rule>) {
    let elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::ID => println!("iei : id"),
        _ => unreachable!(),
    }
}

/*
title_elements = {
    (!(NEWLINE | inline_element_id) ~ WS* ~ title_element)+
}
*/
pub fn title_elements(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for elem in elems {
        match elem.as_rule() {
            Rule::inline_element_id => println!("tes : iei"),
            Rule::title_element => println!("tes : te"),
            _ => unreachable!(),
        }
    }
}

/*
    element_attributes?
    ~ "="{1,6}
    ~ title_elements ~ inline_element_id* ~ EOL
*/
pub fn section_header(ast: Pair<Rule>) {
    let mut c = ast.into_inner();
    for elem in c {
        match elem.as_rule() {
            Rule::element_attributes => element_attributes(elem),
            Rule::title_elements => title_elements(elem),
            Rule::inline_element_id => inline_element_id(elem),
            _ => unreachable!(),
        }
    }
}

pub fn delimited_block(ast: Pair<Rule>) {
    let mut elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::fenced_block => fenced_block(elem),
        Rule::listing_block => listing_block(elem),
        Rule::example_block => example_block(elem),
        Rule::verse_block => verse_block(elem),
        Rule::quote_block => quote_block(elem),
        Rule::sidebar_block => sidebar_block(elem),
        Rule::single_line_comment => single_line_comment(elem),
        Rule::table => table(elem),
        Rule::comment_block => comment_block(elem),
        _ => unreachable!(),
    }
}

/*
fenced_block = {
    element_attributes?
    ~ fenced_block_delimiter
    ~ fenced_block_content*
    ~ (fenced_block_delimiter | EOI)
}
*/
pub fn fenced_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => element_attributes(e),
            Rule::fenced_block_delimiter => println!("fenced block ----"),
            Rule::fenced_block_content => fenced_block_content(e),
            _ => unreachable!(),
        }
    }
}

// fenced_block_content = {
//   blank_line | file_inclusion | list_item | fenced_block_paragraph
// }
pub fn fenced_block_content(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::blank_line => println!("blank line"),
        Rule::file_inclusion => file_inclusion(e),
        Rule::list_item => list_item(e),
        Rule::fenced_block_paragraph => fenced_block_paragraph(e),
        _ => unreachable!(),
    }
}

// fenced_block_paragraph = { fenced_block_paragraph_line+ }
pub fn fenced_block_paragraph(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::fenced_block_paragraph_line => fenced_block_paragraph_line(e),
            _ => unreachable!(),
        }
    }
}

pub fn fenced_block_paragraph_line(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::inline_elements => inline_elements(e),
            _ => unreachable!(),
        }
    }
}

/*
// listing block: verbatim content
listing_block = {
    element_attributes?
    ~ listing_block_delimiter
    ~ listing_block_element*
    ~ (listing_block_delimiter | EOI)
}
*/
pub fn listing_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => element_attributes(e),
            Rule::listing_block_delimiter => {},// do nothing
            Rule::listing_block_element => listing_block_element(e),
            _ => unreachable!(),
        }
    }
}

// listing_block_element = { file_inclusion | listing_block_paragraph }
pub fn listing_block_element(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::file_inclusion => file_inclusion(e),
        Rule::listing_block_paragraph => listing_block_paragraph(e),
        _ => unreachable!(),
    }
}

// listing_block_paragraph = { listing_block_paragraph_line+ }
pub fn listing_block_paragraph(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::listing_block_paragraph_line => listing_block_paragraph_line(e),
            _ => unreachable!(),
        }
    }
}

pub fn listing_block_paragraph_line(ast: Pair<Rule>) {
    // 不用处理，直接拿到文本内容就行了
}

/*
example_block = {
    element_attributes? ~ example_block_delimiter
    ~ (blank_line | file_inclusion | list_item | example_block_paragraph)*
    ~ (example_block_delimiter | EOI)

*/
pub fn example_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => element_attributes(e),
            Rule::example_block_delimiter => {}, // do nothing
            Rule::blank_line => blank_line(e),
            Rule::file_inclusion => file_inclusion(e),
            Rule::list_item => list_item(e),
            Rule::example_block_paragraph => example_block_paragraph(e),
            _ => unreachable!(),
        }
    }
}

// example_block_paragraph = { example_block_paragraph_line+ }
pub fn example_block_paragraph(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::example_block_paragraph_line => example_block_paragraph_line(e),
            _ => unreachable!(),
        }
    }
}

pub fn example_block_paragraph_line(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::inline_elements => inline_elements(e),
            _ => unreachable!(),
        }
    }
}

/*
verse_block = {
    &"[verse" ~ element_attributes
    ~ quote_block_delimiter ~ verse_block_element*
    ~ (quote_block_delimiter | EOI)
}
*/
pub fn verse_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => element_attributes(e),
            Rule::quote_block_delimiter => {}, // do nothing
            Rule::verse_block_element => verse_block_element(e),
            _ => unreachable!(),
        }
    }
}

// verse_block_element = { verse_file_include | blank_line | verse_block_paragraph }
pub fn verse_block_element(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::verse_file_include => verse_file_include(e),
        Rule::blank_line => blank_line(e),
        Rule::verse_block_paragraph => verse_block_paragraph(e),
        _ => unreachable!()
    }
}

pub fn verse_file_include(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::file_inclusion => file_inclusion(e),
        _ => unreachable!(),
    }
}

// verse_block_paragraph = { verse_block_paragraph_line+ }
pub fn verse_block_paragraph(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::verse_block_paragraph_line => verse_block_paragraph_line(e),
            _ => unreachable!(),
        }
    }
}

/*
verse_block_paragraph_line = {
    !(quote_block_delimiter | blank_line)
    ~ verse_block_paragraph_line_element+ ~ EOL
}
*/
pub fn verse_block_paragraph_line(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::verse_block_paragraph_line_element => verse_block_paragraph_line_element(e),
            _ => unreachable!(),
        }
    }
}

/*
verse_block_paragraph_line_element = {
        spaces
        | inline_image
        | link
        | passthrough
        | inline_footnote
        | inline_user_macro
        | quoted_text
        | cross_reference
        | document_attribute_substitution
        | inline_element_id
        | other_word
}
*/
pub fn verse_block_paragraph_line_element(ast: Pair<Rule>) {
    //println!("{:?}", ast);
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::spaces => {}, // do nothing
        Rule::inline_image => inline_image(e),
        Rule::link => link(e),
        Rule::passthrough => passthrough(e),
        Rule::inline_footnote => inline_footnote(e),
        Rule::inline_user_macro => inline_user_macro(e),
        Rule::quoted_text => quote_text(e),
        Rule::cross_reference => cross_reference(e),
        Rule::document_attribute_substitution => document_attribute_substitution(e),
        Rule::inline_element_id => inline_element_id(e),
        Rule::other_word => other_word(e),
        _ => unreachable!()
    }
}

// FIXME
pub fn document_attribute_substitution(ast: Pair<Rule>) {
    println!("doc attr substi");
}

// FIXME
pub fn other_word(ast:Pair<Rule>) {
    println!("other word : {}", ast.as_str());
}

/*
quoted_text = {
    bold_text
    | italic_text
    | monospace_text
    | subscript_text
    | superscript_text
    | escaped_bold_text
    | escaped_italic_text
    | escaped_monospace_text
    | escaped_subscript_text
    | escaped_superscript_text
    | subscript_or_superscript_prefix
}
*/
pub fn quote_text(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::bold_text => bold_text(e),
        Rule::italic_text => italic_text(e),
        Rule::monospace_text => monospace_text(e),
        Rule::subscript_text => subscript_text(e),
        Rule::superscript_text => superscript_text(e),
        Rule::escaped_bold_text => escaped_bold_text(e),
        Rule::escaped_italic_text => escaped_italic_text(e),
        Rule::escaped_monospace_text => escaped_monospace_text(e),
        Rule::escaped_subscript_text => escaped_subscript_text(e),
        Rule::escaped_superscript_text => escaped_superscript_text(e),
        Rule::subscript_or_superscript_prefix => subscript_or_superscript_prefix(e),
        _ => unreachable!(),
    }
}

// FIXME
pub fn bold_text(ast: Pair<Rule>) { }
pub fn italic_text(ast: Pair<Rule>) { }
pub fn monospace_text(ast: Pair<Rule>) { }
pub fn subscript_text(ast: Pair<Rule>) { }
pub fn superscript_text(ast: Pair<Rule>) { }
pub fn escaped_bold_text(ast: Pair<Rule>) { }
pub fn escaped_italic_text(ast: Pair<Rule>) { }
pub fn escaped_monospace_text(ast: Pair<Rule>) {}
pub fn escaped_subscript_text(ast: Pair<Rule>) {}
pub fn escaped_superscript_text(ast: Pair<Rule>) {}
pub fn subscript_or_superscript_prefix(ast: Pair<Rule>) {}

// FIXME
pub fn cross_reference(ast: Pair<Rule>) {
    println!("cross ref")
}

// FIXME
pub fn inline_user_macro(ast:Pair<Rule>) {
    println!("inline user macro")
}

/*
inline_footnote = {
    "footnote:[" ~ footnote_content ~ "]"
    | "footnoteref:[" ~ footnote_ref ~ "," ~ footnote_content ~ "]"
    | "footnoteref:[" ~ footnote_ref ~ "]"
}
*/
pub fn inline_footnote(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::footnote_ref => println!("foot note ref"),
            Rule::footnote_content => println!("foot note conc"),
            _ => unreachable!(),
        }
    }
}

// passthrough = { triple_plus_passthrough | single_plus_passthrough | passthrough_macro }
pub fn passthrough(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::triple_plus_passthrough => println!("tri plus pass th"),
        Rule::single_plus_passthrough => println!("sin plus pass th"),
        Rule::passthrough_macro => println!("pass th macro"),
        _ => unreachable!(),
    }
}

// link = { relative_link | external_link }
pub fn link(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::relative_link => println!("rel link"),
        Rule::external_link => println!("ext link"),
        _ => unreachable!(),
    }
}


/*
inline_image = {
    "image:"
    ~ !":"
    ~ URL
    ~ image_attributes
}
*/
pub fn inline_image(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::URL => println!("url"),
            Rule::image_attributes => println!("image attr"),
            _ => unreachable!( )
        }
    }
}

/*
quote_block = {
    element_attributes?
    ~ quote_block_delimiter
    ~ quote_block_element*
    ~ (quote_block_delimiter | EOI)
}
*/
pub fn quote_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => element_attributes(e),
            Rule::quote_block_delimiter => {}, // do nothing
            Rule::quote_block_element => quote_block_element(e),
            _ => unreachable!(),
        }
    }
}

/*
quote_block_element = {
}
*/
pub fn quote_block_element(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::blank_line => blank_line(e),
            Rule::file_inclusion => file_inclusion(e),
            Rule::image_block => image_block(e),
            Rule::list_item => list_item(e),
            Rule::fenced_block => fenced_block(e),
            Rule::listing_block => listing_block(e),
            Rule::example_block => example_block(e),
            Rule::comment_block => comment_block(e),
            Rule::single_line_comment => single_line_comment(e),
            Rule::quote_block => quote_block(e),
            Rule::sidebar_block => sidebar_block(e),
            Rule::table => table(e),
            Rule::literal_block => literal_block(e),
            Rule::document_attribute_declaration => document_attribute_declaration(e),
            Rule::document_attribute_reset => document_attribute_reset(e),
            Rule::table_of_contents_macro => table_of_contents_macro(e),
            Rule::quote_block_paragraph => quote_block_paragraph(e),
            _ => unreachable!(),
        }
    }
}

pub fn quote_block_paragraph(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::inline_elements => inline_elements(e),
            _ => unreachable!(),
        }
    }
}

/*
sidebar_block = {
    element_attributes? ~ sidebar_block_delimiter
    ~ sidebar_block_content*
    ~ (sidebar_block_delimiter | EOI)
}
*/
pub fn sidebar_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => element_attributes(e),
            Rule::sidebar_block_delimiter => {}, // do nothing
            Rule::sidebar_block_content => sidebar_block_content(e),
            _ => unreachable!(),
        }
    }
}

// sidebar_block_content = {
// blank_line
// | file_inclusion
// | list_item
// | non_sidebar_block
// | sidebar_block_paragraph
// }
pub fn sidebar_block_content(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::blank_line => blank_line(e),
        Rule::file_inclusion => file_inclusion(e),
        Rule::list_item => list_item(e),
        Rule::non_sidebar_block => non_sidebar_block(e),
        Rule::sidebar_block_paragraph => sidebar_block_paragraph(e),
        _ => unreachable!(),
    }
}

// non_sidebar_block = { !sidebar_block ~ delimited_block }
pub fn non_sidebar_block(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::delimited_block => delimited_block(e),
        _ => unreachable!(),
    }
}

pub fn sidebar_block_paragraph(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::sidebar_block_paragraph_line => sidebar_block_paragraph_line(e),
            _ => unreachable!(),
        }
    }
}

pub fn sidebar_block_paragraph_line(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::inline_elements => inline_elements(e),
        _ => unreachable!(),
    }
}

/*
single_line_comment = {
    !comment_block_delimiter
    ~ WS*
    ~ "//"
    ~ single_line_comment_content
    ~ EOL
}
*/
pub fn single_line_comment(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        // 这里感觉直接返回文本就可以了
        Rule::single_line_comment_content => println!("single line comment content"),
        _ => unreachable!(),
    }
}

/*
table = {
    element_attributes? ~ table_delimiter
    ~ table_line_header?
    ~ table_line*
    ~ (table_delimiter | EOI)

*/
pub fn table(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => element_attributes(e),
            Rule::table_delimiter =>  {}, // do nothing
            Rule::table_line_header => table_line_header(e),
            Rule::table_line => table_line(e),
            _ => unreachable!(),
        }
    }
}
// table_line = { !table_delimiter ~ table_cell+ ~ EOL ~ blank_line* }
pub fn table_line(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::table_cell => table_cell(e),
            Rule::blank_line => blank_line(e),
            _ => unreachable!(),
        }
    }
}

/*
table_line_header = {
    !table_delimiter ~ table_cell+ ~ EOL ~ blank_line
}
*/
pub fn table_line_header(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::table_cell => table_cell(e),
            Rule::blank_line => blank_line(e),
            _ => unreachable!(),
        }
    }
}

pub fn table_cell(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::inline_element => inline_element(e),
            _ => unreachable!(),
        }
    }
}

/*
inline_element = {
    simple_word
    | spaces
    | inline_image
    | link
    | passthrough
    | inline_footnote
    | inline_user_macro
    | quoted_text
    | cross_reference
    | document_attribute_substitution
    | inline_element_id
    | other_word)
*/
pub fn inline_element(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::simple_word => println!("simple word"),
        Rule::spaces => println!("spaces"),
        Rule::inline_image => inline_image(e),
        Rule::link => link(e),
        Rule::passthrough => passthrough(e),
        Rule::inline_footnote => inline_footnote(e),
        Rule::inline_user_macro => inline_user_macro(e),
        Rule::quoted_text => quote_text(e),
        Rule::cross_reference => cross_reference(e),
        Rule::document_attribute_substitution => document_attribute_substitution(e),
        Rule::inline_element_id => inline_element_id(e),
        Rule::other_word => other_word(e),
        _ => unreachable!(),
    }
}

/*
comment_block = {
    comment_block_delimiter ~ WS* ~ NEWLINE
    ~ comment_block_line*
    ~ ((comment_block_delimiter ~ EOLS) | EOI)
}
*/
pub fn comment_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::comment_block_delimiter => {}, // do nothing
            Rule::NEWLINE => {}, // do nothing?
            Rule::comment_block_line => comment_block_line(e),
            _ => unreachable!(),
        }
    }
}

pub fn comment_block_line(ast:Pair<Rule>) {
    // return string
}

/*
file_inclusion = {
    "include::" ~ file_location ~ file_include_attributes ~ EOLS
}
*/
pub fn file_inclusion(ast: Pair<Rule>) {
    let mut elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::file_location => {
                println!("file inc : file loc");
            }
            Rule::file_include_attributes => {
                println!("file inc : file inc attr");
            }
            _ => unreachable!(),
        }
    }
}

/*
// NOTICE :
// original verse_paragraph use ? after element attributes
verse_paragraph = {
    // admonition paragraph
    ((&"[verse" ~ element_attributes) ~ admonition_kind ~ ": " ~ inline_elements+)
    | ((&"[verse" ~ element_attributes) ~ inline_elements+)
}
*/
pub fn verse_paragraph(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for elem in elems {
        match elem.as_rule() {
            Rule::element_attributes => element_attributes(elem),
            Rule::admonition_kind => println!("verse para : adm kine{}", elem.as_str()),
            Rule::inline_elements => inline_elements(elem),
            _ => unreachable!(),
        }
    }
}

/*
image_block =  {
    element_attributes?
    ~ "image::"
    ~ URL
    ~ image_attributes
    ~ EOLS
}
*/
pub fn image_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for elem in elems {
        match elem.as_rule() {
            Rule::element_attributes => println!("img blo : ele attr"),
            Rule::URL => println!("img blo : url"),
            Rule::image_attributes => println!("img blo : img attr"),
            _ => unreachable!(),
        }
    }
}
// list_item = { ordered_list_item | unordered_list_item | labeled_list_item | continued_list_item_element }
pub fn list_item(ast: Pair<Rule>) {
    let elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::ordered_list_item => println!("list item : olist"),
        Rule::unordered_list_item => println!("list item : ulist"),
        Rule::labeled_list_item => println!("list item : llist"),
        Rule::continued_list_item_element => println!("list item : clist"),
        _ => unreachable!(),
    }
}

pub fn blank_line(ast: Pair<Rule>) {
    // do nothing
}

pub fn literal_block(ast: Pair<Rule>) {
    let elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::paragraph_with_literal_attribute => println!("lit blo : pla"),
        Rule::paragraph_with_headingspaces => println!("lit blo : ph"),
        Rule::paragraph_with_literal_block_delimiter => println!("lit blo plbd"),
        _ => unreachable!(),
    }
}

/*
document_attribute_declaration = {
    ":" ~ document_attribute_name
    ~ ":" ~ (WS+ ~ document_attribute_value)?
    ~ EOLS
}
*/

pub fn document_attribute_declaration(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for elem in elems {
        match elem.as_rule() {
            Rule::document_attribute_name => println!("dad : dan"),
            Rule::document_attribute_value => println!("dad : dav"),
            _ => unreachable!(),
        }
    }
}

/*
document_attribute_reset = {
    ":!" ~ document_attribute_name ~ ":" ~ EOLS
    | ":" ~ document_attribute_name ~ "!:" ~ EOLS
}
*/
pub fn document_attribute_reset(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for elem in elems {
        match elem.as_rule() {
            Rule::document_attribute_name => println!("doc attr reset : doc attr name"),
            _ => unreachable!(),
        }
    }
}

// table_of_contents_macro = { "toc::[]" ~ EOL }
pub fn table_of_contents_macro(ast: Pair<Rule>) {
    // do nothing currently
}

/*
user_macro_block = {
    user_macro_name ~ "::" ~ user_macro_value ~ user_macro_attributes
}
*/
pub fn user_macro_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for elem in elems {
        match elem.as_rule() {
            Rule::user_macro_name => println!("umb : umn"),
            Rule::user_macro_value => println!("umb : umv"),
            Rule::user_macro_attributes => println!("umb : uma"),
            _ => unreachable!(),
        }
    }
}

pub fn paragraph(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for elem in elems {
        match elem.as_rule() {
            Rule::element_attributes => element_attributes(elem),
            Rule::admonition_kind => println!("para : ak"),
            Rule::inline_elements => inline_elements(elem),
            _ => unreachable!(),
        }
    }
}

pub fn sections(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::section => section(e),
            _ => unreachable!(),
        }
    }
}

pub fn section(ast: Pair<Rule>) {
    let elem = ast.into_inner().next().unwrap();
    println!("doc block : {:?}", elem.as_rule());
    match elem.as_rule() {
        Rule::simple_paragraph => simple_paragraph(elem),
        Rule::delimited_block => delimited_block(elem),
        Rule::file_inclusion => file_inclusion(elem),
        Rule::verse_paragraph => verse_paragraph(elem),
        Rule::image_block => image_block(elem),
        Rule::list_item => list_item(elem),
        Rule::blank_line => blank_line(elem),
        Rule::literal_block => literal_block(elem),
        Rule::document_attribute_declaration => document_attribute_declaration(elem),
        Rule::document_attribute_reset => document_attribute_reset(elem),
        Rule::table_of_contents_macro => table_of_contents_macro(elem),
        Rule::user_macro_block => user_macro_block(elem),
        Rule::paragraph => paragraph(elem),
        _ => unreachable!(),
    }
}

pub fn front_matter(ast: Pair<Rule>) {
    let elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::yaml_front_matter => yaml_front_matter(elem),
        _ => unreachable!(),
    }
}

pub fn yaml_front_matter(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::yaml_front_matter_token => println!("token in yml front matter"), // do nothing
            Rule::yaml_front_matter_content => println!("yaml front matter content"),
            _ => unreachable!(),
        }
    }
}

pub fn pre_flight_document(ast: Pair<Rule>) {
    let mut c = ast.into_inner();
    for elem in c {
        match elem.as_rule() {
            //~ front_matter*
            Rule::front_matter => front_matter(elem),
            //~ document_block
            Rule::document_blocks => document_blocks(elem),
            _ => println!("skip"),
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

fn main() -> Result<(), std::io::Error>{
    let path  = (env::args().nth(1)).unwrap();
    let mut buffer = String::new();
    File::open(Path::new(path.as_str()))?.read_to_string(&mut buffer)?;
    println!("original input is {}", buffer);
    convert(buffer.trim());
    // add toc to str will destroy the ast
    return Ok(());
    /*
    let str = r#"
= title
== second title
content line

[source, go]
----
package main

func main() {
    fmt.Println("hello world")
}

----

== second title 2
* 单个节点代表一个字母
* 如果需要对字符串进行匹配
* 只要从根节点开始依次匹配即可

=== third title
== secomd

1. first level list
2. first level list
3. first level list

[TIP]
====
dancing with eyes
====

[quote,Rūmī]
____
Patience is the key to joy.
____
"#
    .to_string();
    //convert(str);
    */
}

// markdown 风格的 quote 暂时还不支持
/*
> > What's new?
>
> I've got Markdown in my AsciiDoc!
>
> > Like what?
>
> * Blockquotes
> * Headings
> * Fenced code blocks
>
> > Is there more?
>
> Yep. AsciiDoc and Markdown share a lot of common syntax already.
*/

// 这种也不支持
/*
[source,java,subs="verbatim,quotes"]
----
System.out.println("Hello *bold* text").
----
*/

/*
[horizontal]
CPU:: The brain of the computer.
Hard drive:: Permanent storage for operating system and/or user files.
RAM:: Temporarily stores information the CPU uses during operation.



[qanda]
What is Asciidoctor?::
  An implementation of the AsciiDoc processor in Ruby.
What is the answer to the Ultimate Question?:: 42




> I hold it that a little rebellion now and then is a good thing,
> and as necessary in the political world as storms in the physical.
> -- Thomas Jefferson, Papers of Thomas Jefferson: Volume 11

[, James Baldwin]
""
Not everything that is faced can be changed.
But nothing can be changed until it is faced.
""


// 其它的 limitation 可以了解这里
// https://github.com/bytesparadise/libasciidoc/blob/master/LIMITATIONS.adoc
image::trie.png[]

[TIP]
Use abc to do some thing
to know the limit

limitw





http://baidu.com[fsdfasdf]

. dafsdf
. dasfsd
. adfasdf


[width="100%",options="header,footer"]
|====================
| 3 | 3 |  x
| 3 |1  |  xx
|1  |3  |  x
| z | 3 |  z
|====================

fdasfadsf~323~

zxcvxcv^dsafsdf^

.Gettysburg Address
[#gettysburg]
[quote, Abraham Lincoln, Address delivered at the dedication of the Cemetery at Gettysburg]
____
Four score and seven years ago our fathers brought forth
on this continent a new nation...

Now we are engaged in a great civil war, testing whether
that nation, or any nation so conceived and so dedicated,
can long endure. ...
____




.After landing the cloaked Klingon bird of prey in Golden Gate park:
[quote, Captain James T. Kirk, Star Trek IV: The Voyage Home]
Everybody remember where we parked.

.Possible DefOps manual locations
* West wood maze
** Maze heart
*** Reflection pool
** Secret exit
* Untracked file in git repository

[verse, Carl Sandburg, Fog]
____
The fog comes
on little cat feet.

It sits looking
over harbor and city
on silent haunches
and then moves on.
____
*/
