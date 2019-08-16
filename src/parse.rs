extern crate pest;

use pest::iterators::Pair;
use pest::{Parser, RuleType};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExprParser;

/// error occurred when parsing user input
#[derive(Debug)]
pub struct ParseError {
    pub location: pest::error::InputLocation,
    pub expected: String,
}

pub fn document_blocks(ast: Pair<Rule>) -> String {
    let mut result = String::new();

    let elems = ast.into_inner();
    for e in elems {
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
    let elems = ast.into_inner();
    for e in elems {
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

pub fn preamble(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::section_body => {
                result += section_body(e).as_str();
            }
            _ => unreachable!(),
        }
    }
    result
}

pub fn section_body(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::table_of_contents_macro => {
                table_of_contents_macro(e);
            }
            Rule::document_attribute_declaration => {
                document_attribute_declaration(e);
            }
            Rule::document_attribute_reset => {
                document_attribute_reset(e);
            }
            Rule::user_macro_block => {
                user_macro_block(e);
            }
            Rule::blank_line => {}
            Rule::literal_block => {
                literal_block(e);
            }
            Rule::delimited_block => {
                delimited_block(e);
            }
            Rule::file_inclusion => {
                file_inclusion(e);
            }
            Rule::image_block => image_block(e),
            Rule::list_items => list_items(e),
            Rule::paragraph => {
                result = result + "\n" + paragraph(e).as_str();
            }
            _ => unreachable!(),
        }
    }
    result
}

pub fn listing_paragraph(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => {
                element_attributes(e);
            }
            Rule::inline_elements => inline_elements(e),
            _ => unreachable!(),
        }
    }
}

pub fn source_paragraph(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => {
                element_attributes(e);
            }
            Rule::inline_elements => inline_elements(e),
            _ => unreachable!(),
        }
    }
}

pub fn list_items(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::list_item => list_item(e),
            _ => unreachable!(),
        }
    }
}

// element_attributes = { element_attribute+ }
pub fn element_attributes(ast: Pair<Rule>) -> Vec<ElementAttribute> {
    let mut result = vec![];
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attribute => {
                result.push(element_attribute(e));
            }
            _ => unreachable!(),
        }
    }

    result
}

pub enum ElementAttribute {
    ElementID(String),
    ElementTitle(String),
    ElementRole(String),
    LiteralAttribute,
    SourceAttributes(String),
    QuoteAttributes(String),
    VerseAttributes(String),
    AdmonitionMarker(String),
    Horizontal(String),
    AttributeGroup(String),
}

pub fn element_attribute(ast: Pair<Rule>) -> ElementAttribute {
    let elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::element_id => return ElementAttribute::ElementID(elem.to_string()),
        Rule::element_title => return ElementAttribute::ElementTitle(elem.to_string()),
        Rule::element_role => return ElementAttribute::ElementRole(elem.to_string()),
        Rule::literal_attribute => return ElementAttribute::LiteralAttribute,
        Rule::source_attributes => return ElementAttribute::SourceAttributes(elem.to_string()),
        Rule::quote_attributes => return ElementAttribute::QuoteAttributes(elem.to_string()),
        Rule::verse_attributes => return ElementAttribute::VerseAttributes(elem.to_string()),
        Rule::admonition_marker_attribute => {
            return ElementAttribute::AdmonitionMarker(elem.to_string());
        }
        Rule::horizontal_layout => return ElementAttribute::Horizontal(elem.to_string()),
        Rule::attribute_group => return ElementAttribute::AttributeGroup(elem.to_string()),
        _ => unreachable!(),
    }
}

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
    let c = ast.into_inner();
    for elem in c {
        match elem.as_rule() {
            Rule::element_attributes => {
                element_attributes(elem);
            }
            Rule::first_paragraph_line => first_paragraph_line(elem),
            Rule::other_paragraph_line => other_paragraph_line(elem),
            _ => unreachable!(),
        }
    }
}

pub fn document_header(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    let elems = ast.into_inner();
    for e in elems {
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

pub fn document_authors(ast: Pair<Rule>) {
    let elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::document_authors_inline_form => println!("da : daif"),
        Rule::document_authors_attribute_form => println!("da: daaf"),
        _ => unreachable!(),
    }
}

pub fn inline_element_id(ast: Pair<Rule>) {
    let elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::ID => println!("iei : id"),
        _ => unreachable!(),
    }
}

pub fn title_elements(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for elem in elems {
        match elem.as_rule() {
            Rule::inline_element_id => println!("title elem, inline elem id : {}", elem.as_str()),
            Rule::title_element => println!("tltle elem : {}", elem.as_str()),
            _ => unreachable!(),
        }
    }
}

pub fn section_header(ast: Pair<Rule>) -> SectionHeader {
    let mut result = String::new();
    let mut level = 0;
    let c = ast.clone().into_inner();
    for elem in c {
        match elem.as_rule() {
            Rule::element_attributes => {
                element_attributes(elem);
            }
            Rule::title_elements => {
                // 因为 === 规则是静默的
                // 但是需要知道这里是 level 几的 header
                // 所以要读外层的字符串
                level = ast.as_str().chars().take_while(|x| *x == '=').count();
                let (start_tag, end_tag) = (format!("<h{}>", level), format!("</h{}>", level));
                result = format!("{}{}{}{}", result, start_tag, elem.as_str(), end_tag);
            }
            Rule::inline_element_id => inline_element_id(elem),
            _ => unreachable!(),
        }
    }

    SectionHeader {
        level: level,
        title: result,
    }
}

pub fn delimited_block(ast: Pair<Rule>) -> String {
    let mut result = String::new();

    let elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::fenced_block => fenced_block(elem),
        Rule::listing_block => {
            result += listing_block(elem).as_str();
        }
        Rule::example_block => example_block(elem),
        Rule::verse_block => verse_block(elem),
        Rule::quote_block => quote_block(elem),
        Rule::sidebar_block => sidebar_block(elem),
        Rule::single_line_comment => single_line_comment(elem),
        Rule::table => table(elem),
        Rule::comment_block => comment_block(elem),
        _ => unreachable!(),
    }

    result
}

pub fn fenced_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => {
                element_attributes(e);
            }
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

pub fn listing_block(ast: Pair<Rule>) -> String {
    let mut result = String::new();

    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => {
                //element_attributes(e);
                let attrs = e.into_inner();
                for attr in attrs {
                    element_attribute(attr);
                }
            }
            Rule::listing_block_delimiter => {} // do nothing
            Rule::listing_block_element => {
                result += listing_block_element(e).as_str();
            }
            _ => unreachable!(),
        }
    }

    // todo, listing block 内部的 result
    // 需要根据 element attr 修改样式
    format!(
        "{}{}{}{}{}",
        r#"<div class="listingblock">"#, r#"<div class="content">"#, result, "</div>", r"</div>"
    )
}

// listing_block_element = { file_inclusion | listing_block_paragraph }
pub fn listing_block_element(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::file_inclusion => file_inclusion(e),
        Rule::listing_block_paragraph => {
            //listing_block_paragraph(e)
            result += e.as_str();
        }
        _ => unreachable!(),
    }

    result
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

pub fn example_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => {
                element_attributes(e);
            }
            Rule::example_block_delimiter => {} // do nothing
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

pub fn verse_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => {
                element_attributes(e);
            }
            Rule::quote_block_delimiter => {} // do nothing
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
        _ => unreachable!(),
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

pub fn verse_block_paragraph_line(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::verse_block_paragraph_line_element => verse_block_paragraph_line_element(e),
            _ => unreachable!(),
        }
    }
}

pub fn verse_block_paragraph_line_element(ast: Pair<Rule>) {
    //println!("{:?}", ast);
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::spaces => {} // do nothing
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

// FIXME
pub fn document_attribute_substitution(ast: Pair<Rule>) {
    println!("doc attr substi");
}

// FIXME
pub fn other_word(ast: Pair<Rule>) {
    println!("other word : {}", ast.as_str());
}

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
pub fn bold_text(ast: Pair<Rule>) {}
pub fn italic_text(ast: Pair<Rule>) {}
pub fn monospace_text(ast: Pair<Rule>) {}
pub fn subscript_text(ast: Pair<Rule>) {}
pub fn superscript_text(ast: Pair<Rule>) {}
pub fn escaped_bold_text(ast: Pair<Rule>) {}
pub fn escaped_italic_text(ast: Pair<Rule>) {}
pub fn escaped_monospace_text(ast: Pair<Rule>) {}
pub fn escaped_subscript_text(ast: Pair<Rule>) {}
pub fn escaped_superscript_text(ast: Pair<Rule>) {}
pub fn subscript_or_superscript_prefix(ast: Pair<Rule>) {}

// FIXME
pub fn cross_reference(ast: Pair<Rule>) {
    println!("cross ref")
}

// FIXME
pub fn inline_user_macro(ast: Pair<Rule>) {
    println!("inline user macro")
}

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

pub fn inline_image(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::URL => println!("url"),
            Rule::image_attributes => println!("image attr"),
            _ => unreachable!(),
        }
    }
}

pub fn quote_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => {
                element_attributes(e);
            }
            Rule::quote_block_delimiter => {} // do nothing
            Rule::quote_block_element => quote_block_element(e),
            _ => unreachable!(),
        }
    }
}

pub fn quote_block_element(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::blank_line => blank_line(e),
            Rule::file_inclusion => file_inclusion(e),
            Rule::image_block => image_block(e),
            Rule::list_item => list_item(e),
            Rule::fenced_block => fenced_block(e),
            Rule::listing_block => {
                listing_block(e);
            }
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

pub fn sidebar_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attributes => {
                element_attributes(e);
            }
            Rule::sidebar_block_delimiter => {} // do nothing
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
        Rule::delimited_block => {
            delimited_block(e);
        }
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

pub fn single_line_comment(ast: Pair<Rule>) {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        // 这里感觉直接返回文本就可以了
        Rule::single_line_comment_content => println!("single line comment content"),
        _ => unreachable!(),
    }
}

pub fn table(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
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
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::table_cell => table_cell(e),
            Rule::blank_line => blank_line(e),
            _ => unreachable!(),
        }
    }
}

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

pub fn comment_block(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::comment_block_delimiter => {} // do nothing
            Rule::NEWLINE => {}                 // do nothing?
            Rule::comment_block_line => comment_block_line(e),
            _ => unreachable!(),
        }
    }
}

pub fn comment_block_line(ast: Pair<Rule>) {
    // return string
}

pub fn file_inclusion(ast: Pair<Rule>) {
    let elems = ast.into_inner();
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

pub fn verse_paragraph(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for elem in elems {
        match elem.as_rule() {
            Rule::element_attributes => {
                element_attributes(elem);
            }
            Rule::admonition_kind => println!("verse para : adm kine{}", elem.as_str()),
            Rule::inline_elements => inline_elements(elem),
            _ => unreachable!(),
        }
    }
}

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

pub fn paragraph(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    let elems = ast.into_inner();
    for elem in elems {
        match elem.as_rule() {
            Rule::element_attributes => {
                element_attributes(elem);
            }
            Rule::admonition_kind => println!("para : ak"),
            Rule::inline_elements => {
                //inline_elements(elem)
                result += elem.as_str();
            }
            _ => unreachable!(),
        }
    }
    result = format!(
        "{}{}{}",
        r#"<div class="paragraph"><p>"#, result, "</p></div>"
    );
    result
}

pub struct SectionHeader {
    level: usize,
    title: String,
}

#[derive(Debug, Clone)]
pub struct Section {
    level: usize,
    content: String,
}

pub fn sections(ast: Pair<Rule>) -> String {
    let mut section_list = vec![];
    let mut result = String::new();

    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::section => {
                let sec = section(e);
                section_list.push(sec.clone());
                result = format!("{}{}\n", result, sec.content);
            }
            _ => unreachable!(),
        }
    }

    //section_list
    result
}

pub fn section(ast: Pair<Rule>) -> Section {
    let mut result = String::new();
    let elems = ast.into_inner();
    let mut body = String::new();
    let mut header = SectionHeader {
        level: 0,
        title: "".to_string(),
    };

    let mut body_list = vec![];

    for e in elems {
        match e.as_rule() {
            Rule::section_header => {
                header = section_header(e);
            }
            Rule::section_body => {
                body_list.push(section_body(e));
            }
            _ => unreachable!(),
        }
    }

    Section {
        level: header.level,
        content: format!(
            "{}\n{}{}{}",
            header.title,
            r#"<div class="section_body">"#,
            body_list.join("").as_str(),
            r#"</div>"#,
        ),
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
