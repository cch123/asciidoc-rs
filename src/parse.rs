use crate::block::*;
use crate::structs::BlockType::{AdmonitionBlock, ExampleBlock, NotBlock};
use crate::structs::*;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExprParser;

pub fn elem_attrs(elems: Vec<Pair<Rule>>) -> Block {
    let mut b = Block {
        id: "".to_string(),
        role: "".to_string(),
        title: "".to_string(),
        block_type: BlockType::NotBlock,
    };

    for elem in elems {
        match elem.as_rule() {
            Rule::element_attribute => {
                if let Some(e) = elem.into_inner().next() {
                    match e.as_rule() {
                        Rule::element_id => {
                            b.id = e.as_str().to_string();
                        }
                        Rule::element_role => {
                            b.role = e.as_str().to_string();
                        }
                        Rule::element_title => {
                            b.title = e.as_str().to_string();
                        }
                        Rule::literal_attribute => {
                            b.block_type = BlockType::LiteralBlock;
                        }
                        Rule::source_attributes => {
                            let lang = if let Some(source_lang) = e.into_inner().next() {
                                source_lang.as_str().to_string()
                            } else {
                                "c".to_string()
                            };
                            b.block_type = BlockType::SourceBlock { lang };
                        }
                        Rule::quote_attributes => {
                            let (mut author, mut source) = (String::new(), String::new());
                            for e_in in e.into_inner() {
                                match e_in.as_rule() {
                                    Rule::quote_author => author = e_in.as_str().to_string(),
                                    Rule::quote_source => source = e_in.as_str().to_string(),
                                    _ => unreachable!(),
                                }
                            }
                            b.block_type = BlockType::QuoteBlock { author, source };
                        }
                        Rule::verse_attributes => {
                            let (mut author, mut source) = (String::new(), String::new());
                            for e_in in e.into_inner() {
                                match e_in.as_rule() {
                                    Rule::quote_author => author = e_in.as_str().to_string(),
                                    Rule::quote_source => source = e_in.as_str().to_string(),
                                    _ => unreachable!(),
                                }
                            }
                            b.block_type = BlockType::VerseBlock { author, source };
                        }
                        Rule::admonition_marker_attribute => {
                            let marker = e.into_inner().next().unwrap().as_str().to_string();
                            b.block_type = BlockType::AdmonitionBlock { marker };
                        }
                        //Rule::attribute_group
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    b
}

// element_attributes = { element_attribute+ }
pub fn element_attributes(ast: Pair<Rule>) -> Block {
    let mut param = vec![];
    for e in ast.into_inner() {
        param.push(e);
    }

    elem_attrs(param)
}

pub fn first_paragraph_line(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::simple_word => println!("simple word"),
            Rule::inline_element => println!("inline ele 1"),
            Rule::line_break => println!("line break"),
            _ => unreachable!(),
        }
    }
}

pub fn inline_elements(ast: Pair<Rule>) -> String {
    let mut content = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::single_line_comment => println!("single_line cmt"),
            Rule::inline_element => content += inline_element(e).as_str(),
            _ => unreachable!(),
        }
    }
    content
}

pub fn other_paragraph_line(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::inline_elements => {
                inline_elements(e);
            }
            _ => unreachable!(),
        }
    }
}

/*
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
*/

pub fn document_authors(ast: Pair<Rule>) {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            Rule::document_authors_inline_form => println!("da : daif"),
            Rule::document_authors_attribute_form => println!("da: daaf"),
            _ => unreachable!(),
        }
    }
}

pub fn inline_element_id(ast: Pair<Rule>) {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            Rule::ID => println!("iei : id"),
            _ => unreachable!(),
        }
    }
}

pub fn title_elements(ast: Pair<Rule>) {
    for elem in ast.into_inner() {
        match elem.as_rule() {
            Rule::inline_element_id => println!("title elem, inline elem id : {}", elem.as_str()),
            Rule::title_element => println!("tltle elem : {}", elem.as_str()),
            _ => unreachable!(),
        }
    }
}

// FIXME
pub fn document_attribute_substitution(_ast: Pair<Rule>) {
    println!("doc attr substi");
}

// FIXME
pub fn other_word(ast: Pair<Rule>) {
    println!("other word : {}", ast.as_str());
}

pub fn quote_text(ast: Pair<Rule>) {
    if let Some(e) = ast.into_inner().next() {
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
}

// FIXME
pub fn bold_text(_ast: Pair<Rule>) {}
pub fn italic_text(_ast: Pair<Rule>) {}
pub fn monospace_text(_ast: Pair<Rule>) {}
pub fn subscript_text(_ast: Pair<Rule>) {}
pub fn superscript_text(_ast: Pair<Rule>) {}
pub fn escaped_bold_text(_ast: Pair<Rule>) {}
pub fn escaped_italic_text(_ast: Pair<Rule>) {}
pub fn escaped_monospace_text(_ast: Pair<Rule>) {}
pub fn escaped_subscript_text(_ast: Pair<Rule>) {}
pub fn escaped_superscript_text(_ast: Pair<Rule>) {}
pub fn subscript_or_superscript_prefix(_ast: Pair<Rule>) {}

// FIXME
pub fn cross_reference(_ast: Pair<Rule>) {
    println!("cross ref")
}

// FIXME
pub fn inline_user_macro(_ast: Pair<Rule>) {
    println!("inline user macro")
}

pub fn inline_footnote(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::footnote_ref => println!("foot note ref"),
            Rule::footnote_content => println!("foot note conc"),
            _ => unreachable!(),
        }
    }
}

// passthrough = { triple_plus_passthrough | single_plus_passthrough | passthrough_macro }
pub fn passthrough(ast: Pair<Rule>) {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            Rule::triple_plus_passthrough => println!("tri plus pass th"),
            Rule::single_plus_passthrough => println!("sin plus pass th"),
            Rule::passthrough_macro => println!("pass th macro"),
            _ => unreachable!(),
        }
    }
}

// link = { relative_link | external_link }
pub fn link(ast: Pair<Rule>) {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            Rule::relative_link => println!("rel link"),
            Rule::external_link => println!("ext link"),
            _ => unreachable!(),
        }
    }
}

pub fn inline_image(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::URL => println!("url"),
            Rule::image_attributes => println!("image attr"),
            _ => unreachable!(),
        }
    }
}

pub fn single_line_comment(ast: Pair<Rule>) {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            // 这里感觉直接返回文本就可以了
            Rule::single_line_comment_content => println!("single line comment content"),
            _ => unreachable!(),
        }
    }
}

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

pub fn inline_element(ast: Pair<Rule>) -> String {
    return ast
        .clone()
        .into_inner()
        .next()
        .unwrap()
        .as_str()
        .to_string();
    /*
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::simple_word => {
            println!("simple word");
        },
        Rule::spaces => {
            println!("spaces");
        },
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
    }*/
}

pub fn file_inclusion(ast: Pair<Rule>) -> String {
    for e in ast.clone().into_inner() {
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

    ast.as_str().to_string()
}

// list_item = { ordered_list_item | unordered_list_item | labeled_list_item | continued_list_item_element }
/*
pub fn list_item(ast: Pair<Rule>) {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            Rule::ordered_list_item => println!("list item : olist"),
            Rule::unordered_list_item => println!("list item : ulist"),
            Rule::labeled_list_item => println!("list item : llist"),
            Rule::continued_list_item_element => println!("list item : clist"),
            _ => unreachable!(),
        }
    }
}
*/

pub fn blank_line(_ast: Pair<Rule>) {
    // do nothing
}

pub fn paragraph_with_literal_block_delimiter(ast: Pair<Rule>) -> String {
    let mut tpl =
        r#"<div #id_holder class="literalblock">#title_holder<div class="content"><pre>#place_holder</pre></div></div>"#
            .to_string();
    let mut content = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::element_attributes => {
                let b = element_attributes(e);
                tpl = get_listing_block_tpl(b, tpl);
            }
            Rule::paragraph_with_literal_block_delimiter_lines => {
                content += e.as_str();
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    tpl.replace("#place_holder", content.as_str())
}

pub fn paragraph_with_headingspaces(ast: Pair<Rule>) -> String {
    let mut tpl =
        r#"<div #id_holder class="literalblock">#title_holder<div class="content"><pre>#place_holder</pre></div></div>"#
            .to_string();
    let mut content = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::element_attributes => {
                let mut b = element_attributes(e);
                // the block type should always be literal block
                b.block_type = BlockType::LiteralBlock;
                tpl = get_listing_block_tpl(b, tpl);
            }
            Rule::paragraph_with_headingspaces_lines => {
                content += e.as_str().trim_start();
            }
            _ => unreachable!(),
        }
    }

    tpl.replace("#place_holder", content.as_str())
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

pub fn user_macro_block(ast: Pair<Rule>) {
    for elem in ast.into_inner() {
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
    let mut tpl =
        r#"<div #id_holder class="paragraph">#title_holder<p>#place_holder</p></div>"#.to_string();
    let mut b = Block {
        id: "".to_string(),
        role: "".to_string(),
        title: "".to_string(),
        block_type: BlockType::NotBlock,
    };

    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::element_attributes => {
                let block = element_attributes(e);
                if !block.title.is_empty() {
                    b.title = block.title
                }
                if !block.role.is_empty() {
                    b.role = block.role
                };
                if !block.id.is_empty() {
                    b.id = block.id
                };
                b.block_type = block.block_type;
            }
            Rule::admonition_kind => {
                b.block_type = AdmonitionBlock {
                    marker: e.as_str().to_string(),
                };
            }
            Rule::inline_elements => {
                //inline_elements(elem)
                result += e.as_str();
            }
            _ => unreachable!(),
        }
    }

    tpl = get_listing_block_tpl(b, tpl);

    tpl.replace("#place_holder", result.as_str())
        .replace("#id_holder", "")
        .replace("#title_holder", "")
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
