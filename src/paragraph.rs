use crate::block::*;
use crate::parse::*;
use crate::structs::*;
use pest::iterators::Pair;

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
        .replace("#id_holder", "")
        .replace("#title_holder", "")
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
        .replace("#id_holder", "")
        .replace("#title_holder", "")
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
                b.block_type = BlockType::AdmonitionBlock {
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
