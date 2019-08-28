use crate::block::*;
use crate::list::*;
use crate::parse::*;
use crate::structs::*;
use pest::iterators::Pair;

pub fn sections(ast: Pair<Rule>) -> String {
    let mut section_list = vec![];
    let mut result = String::new();

    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::section => {
                let sec = section(e);
                section_list.push(sec.clone());
                result = format!("{}{}\n", result, sec.content);
            }
            _ => unreachable!(),
        }
    }

    result
}

pub fn section(ast: Pair<Rule>) -> Section {
    //let result = String::new();
    //let body = String::new();
    let mut header = SectionHeader {
        level: 0,
        title: "".to_string(),
    };

    let mut body_list = vec![];

    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::section_header => {
                header = section_header(e);
            }
            Rule::section_body => {
                let body = section_body(e);
                if !body.is_empty() {
                    body_list.push(body);
                }
            }
            _ => unreachable!(),
        }
    }

    Section {
        level: header.level,
        content: format!(
            "{}{}{}{}",
            header.title,
            r#"<div class="section_body">"#,
            body_list.join("").as_str(),
            r#"</div>"#,
        ),
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
        level,
        title: result,
    }
}

pub fn section_body(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    for e in ast.into_inner() {
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
            Rule::blank_line => {} // do nothing
            Rule::literal_block => {
                result += literal_block(e).as_str();
            }
            Rule::delimited_block => {
                result += delimited_block(e).as_str();
            }
            Rule::file_inclusion => {
                file_inclusion(e);
            }
            Rule::image_block => {
                result += image_block(e).as_str();
            }
            Rule::list_items => {
                result += list_items(e).as_str();
            }
            Rule::paragraph => {
                result += paragraph(e).as_str();
            }
            _ => unreachable!(),
        }
    }
    result
}

pub fn preamble(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::section_body => {
                result += section_body(e).as_str();
            }
            _ => unreachable!(),
        }
    }
    result
}
