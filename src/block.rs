use crate::document::*;
use crate::list::*;
use crate::paragraph::*;
use crate::parse::*;
use crate::structs::*;
use crate::table::*;
use pest::iterators::Pair;

pub fn delimited_block(ast: Pair<Rule>) -> String {
    let mut result = String::new();

    if let Some(elem) = ast.into_inner().next() {
        match elem.as_rule() {
            Rule::fenced_block => result += fenced_block(elem).as_str(),
            Rule::listing_block => result += listing_block(elem).as_str(),
            Rule::example_block => result += example_block(elem).as_str(),
            Rule::verse_block => result += verse_block(elem).as_str(),
            Rule::quote_block => result += quote_block(elem).as_str(),
            Rule::sidebar_block => result += sidebar_block(elem).as_str(),
            Rule::single_line_comment => single_line_comment(elem),
            Rule::table => table(elem),
            Rule::comment_block => {} // do nothing
            _ => unreachable!(),
        }
    }

    result
}

pub fn fenced_block(ast: Pair<Rule>) -> String {
    let mut tpl = r#"<div #id_holder class="listingblock">#title_holder<div class="content"><pre class="highlight"><code>#place_holder</code></pre></div></div>"#.to_string();
    let mut content = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::element_attributes => {
                let mut b = element_attributes(e);
                // 强制使用 source block
                b.block_type = BlockType::SourceBlock {
                    lang: "".to_string(),
                };
                tpl = get_listing_block_tpl(b, tpl);
            }
            Rule::fenced_block_delimiter => {} // do nothing
            Rule::fenced_block_content => content = fenced_block_content(e),
            _ => unreachable!(),
        }
    }

    tpl.replace("#place_holder", content.as_str().trim())
}
// fenced_block_content = {
//   blank_line | file_inclusion | list_item | fenced_block_paragraph
// }
pub fn fenced_block_content(ast: Pair<Rule>) -> String {
    // FIXME
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            /*
            Rule::blank_line
            Rule::file_inclusion
            Rule::list_item
            Rule::fenced_block_paragraph
            */
            _ => return e.as_str().to_string(),
        }
    }
    String::new()
}

// fenced_block_paragraph = { fenced_block_paragraph_line+ }
pub fn fenced_block_paragraph(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::fenced_block_paragraph_line => {
                fenced_block_paragraph_line(e);
            }
            _ => unreachable!(),
        }
    }
}

pub fn fenced_block_paragraph_line(ast: Pair<Rule>) -> String {
    let mut content = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::inline_elements => {
                content += inline_elements(e).as_str();
            }
            _ => unreachable!(),
        }
    }
    content
}

pub fn listing_block(ast: Pair<Rule>) -> String {
    // 如果发现是 source、literal、verse、quote
    // 需要替换掉这里的类型
    // let mut elem_type = ElementAttribute::LiteralAttribute;

    let mut tpl =
        r#"<div #id_holder class="listingblock">#title_holder<div class="content">#place_holder</div></div>"#.to_string();

    let mut content = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::element_attributes => {
                // step 1 通过 element attr 获取 block 的属性和模板
                let block = element_attributes(e);
                tpl = get_listing_block_tpl(block, tpl);
            }
            Rule::listing_block_element => {
                // step 2 向 tpl 中填充内容
                content += listing_block_element(e).as_str();
            }
            Rule::listing_block_delimiter | Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    tpl.replace("#place_holder", content.as_str())
        .replace("#id_holder", "")
        .replace("#title_holder", "")
}

// listing_block_element = { file_inclusion | listing_block_paragraph }
pub fn listing_block_element(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            Rule::file_inclusion => {
                result += file_inclusion(e).as_str();
            }
            Rule::listing_block_paragraph => {
                //listing_block_paragraph(e)
                result += e.as_str();
            }
            _ => unreachable!(),
        }
    }

    result
}

// listing_block_paragraph = { listing_block_paragraph_line+ }
pub fn listing_block_paragraph(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::listing_block_paragraph_line => listing_block_paragraph_line(e),
            _ => unreachable!(),
        }
    }
}

pub fn listing_block_paragraph_line(_ast: Pair<Rule>) {
    // 不用处理，直接拿到文本内容就行了
}

pub fn example_block(ast: Pair<Rule>) -> String {
    let mut content = String::new();
    let mut tpl =
        r#"<div #id_holder class="exampleblock">#title_holder<div class="content">#place_holder</div></div>"#.to_string();

    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::element_attributes => {
                let mut b = element_attributes(e);
                match &b.block_type {
                    BlockType::AdmonitionBlock { marker: _ } => {}
                    _ => b.block_type = BlockType::ExampleBlock,
                }

                tpl = get_listing_block_tpl(b, tpl);
            }
            // TODO
            Rule::blank_line => blank_line(e),
            Rule::file_inclusion => {
                content += file_inclusion(e).as_str();
            }
            Rule::list_items => {
                content += list_items(e).as_str();
            }
            // TODO
            Rule::example_block_paragraph => content += example_block_paragraph(e).as_str(),
            Rule::example_block_delimiter => {} // do nothing
            Rule::example_block => content += example_block(e).as_str(),
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    tpl.replace("#place_holder", content.as_str())
        .replace("#id_holder", "")
        .replace("#title_holder", "")
}

// example_block_paragraph = { example_block_paragraph_line+ }
pub fn example_block_paragraph(ast: Pair<Rule>) -> String {
    for e in ast.clone().into_inner() {
        match e.as_rule() {
            Rule::example_block_paragraph_line => example_block_paragraph_line(e),
            _ => unreachable!(),
        }
    }

    format!(r#"<div class="paragraph"><p>{}</p></div>"#, ast.as_str())
}

pub fn example_block_paragraph_line(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::inline_elements => {
                inline_elements(e);
            }
            _ => unreachable!(),
        }
    }
}

pub fn verse_block(ast: Pair<Rule>) -> String {
    // fix me
    let mut tpl = "".to_string();
    let mut content = String::new();

    let mut elem_attr_list = vec![];

    for e in ast.into_inner() {
        match e.as_rule() {
            // verse block 可能会进入两次
            Rule::element_attribute => {
                elem_attr_list.push(e);
            }
            Rule::verse_block_element => {
                content += verse_block_element(e).as_str();
            }
            Rule::quote_block_delimiter => {} // do nothing
            _ => unreachable!(),
        }
    }

    let merged_block = elem_attrs(elem_attr_list);

    tpl = get_listing_block_tpl(merged_block, tpl);

    tpl.replace("#place_holder", content.as_str())
}

pub fn quote_block(ast: Pair<Rule>) -> String {
    // fix me?
    let mut tpl = String::new();
    let mut content = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::element_attributes => {
                let b = element_attributes(e);
                tpl = get_listing_block_tpl(b, tpl);
            }
            Rule::quote_block_element => {
                content += format!(
                    r#"<div class="paragraph"><p>{}</p></div>"#,
                    quote_block_element(e).as_str()
                )
                .as_str();
            }
            Rule::quote_block_delimiter => {} // do nothing
            _ => unreachable!(),
        }
    }

    tpl.replace("#place_holder", content.as_str())
}

pub fn quote_block_element(ast: Pair<Rule>) -> String {
    ast.as_str().to_string()
    // TODO
    /*
    for e in ast.clone().into_inner() {
        match e.as_rule() {
            Rule::blank_line => blank_line(e),
            Rule::file_inclusion => file_inclusion(e),
            Rule::image_block => image_block(e),
            Rule::list_item => list_item(e),
            Rule::fenced_block => {
                fenced_block(e);
            }
            Rule::listing_block => {
                listing_block(e);
            }
            Rule::example_block => example_block(e),
            Rule::comment_block => {} // do nothing
            Rule::single_line_comment => single_line_comment(e),
            Rule::quote_block => {
                quote_block(e);
            }
            Rule::sidebar_block => sidebar_block(e),
            Rule::table => table(e),
            Rule::literal_block => {
                literal_block(e);
            }
            Rule::document_attribute_declaration => document_attribute_declaration(e),
            Rule::document_attribute_reset => document_attribute_reset(e),
            Rule::table_of_contents_macro => table_of_contents_macro(e),
            Rule::quote_block_paragraph => quote_block_paragraph(e),
            _ => unreachable!(),
        }
    }
    */
}

pub fn quote_block_paragraph(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::inline_elements => {
                inline_elements(e);
            }
            _ => unreachable!(),
        }
    }
}

pub fn sidebar_block(ast: Pair<Rule>) -> String {
    let mut tpl =
        r#"<div #id_holder class="sidebarblock">#title_holder<div class="content">#place_holder</div></div>"#.to_string();
    let mut content = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::element_attributes => {
                let mut b = element_attributes(e);
                b.block_type = BlockType::SidebarBlock;
                tpl = get_listing_block_tpl(b, tpl);
            }
            Rule::sidebar_block_content => {
                content += format!(
                    r#"<div class="paragraph"><p>{}</p></div>"#,
                    sidebar_block_content(e)
                )
                .as_str();
            }
            Rule::sidebar_block_delimiter => {} // do nothing
            _ => unreachable!(),
        }
    }

    tpl.replace("#place_holder", content.as_str())
        .replace("#id_holder", "")
        .replace("#title_holder", "")
}

// sidebar_block_content = {
// blank_line
// | file_inclusion
// | list_item
// | non_sidebar_block
// | sidebar_block_paragraph
// }
pub fn sidebar_block_content(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    if let Some(e) = ast.clone().into_inner().next() {
        match e.as_rule() {
            Rule::blank_line => blank_line(e),
            // TODO
            Rule::file_inclusion => {
                result += file_inclusion(e).as_str();
            }
            // TODO
            Rule::list_items => result += list_items(e).as_str(),
            Rule::non_sidebar_block => result += non_sidebar_block(e).as_str(),
            Rule::sidebar_block_paragraph => result += sidebar_block_paragraph(e).as_str(),
            _ => unreachable!(),
        }
    }

    result
}

// non_sidebar_block = { !sidebar_block ~ delimited_block }
pub fn non_sidebar_block(ast: Pair<Rule>) -> String {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            Rule::delimited_block => {
                return delimited_block(e);
            }
            _ => unreachable!(),
        }
    }
    String::new()
}

pub fn sidebar_block_paragraph(ast: Pair<Rule>) -> String {
    let mut result = String::new();
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::sidebar_block_paragraph_line => {
                result += sidebar_block_paragraph_line(e).as_str()
            }
            _ => unreachable!(),
        }
    }
    result
}

pub fn sidebar_block_paragraph_line(ast: Pair<Rule>) -> String {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            Rule::inline_elements => return inline_elements(e),
            _ => unreachable!(),
        }
    }
    String::new()
}

pub(crate) fn get_listing_block_tpl(block: Block, default_tpl: String) -> String {
    let (mut block_title_str, mut block_id_str) = (String::new(), String::new());
    if !block.title.is_empty() {
        block_title_str = format!(r#"<div class="title">{}</div>"#, block.title);
    }
    if !block.id.is_empty() {
        block_id_str = format!(r#" id="{}" "#, block.id);
    }
    match block.block_type {
        BlockType::SourceBlock { lang } => {
            let lang = lang.trim();
            match lang.len() {
                0 => return format!(r#"<div {} class="listingblock">{}<div class="content"><pre class="highlight"><code>#place_holder</code></pre></div></div>"#,
                                    block_id_str, block_title_str
                ),
                _ => return format!(
                    r#"<div {} class="listingblock">{}<div class="content"><pre class="highlight">{}</pre></div></div>"#,
                    block_id_str, block_title_str,
                    format!(
                        r#"<code class="language-{}" data-lang="{}">#place_holder</code>"#,
                        lang, lang
                    )
                ),
            }
        }
        BlockType::VerseBlock { author, source } => {
            return format!(r#"<div {} class="verseblock">{}<pre class="content">#place_holder</pre><div class="attribution">—{}<br/><cite>{}</cite></div></div>"#,
                           block_id_str, block_title_str, author, source,
            )
        }
        BlockType::QuoteBlock { author, source } => {
            return format!(r#"<div {} class="quoteblock">{}<blockquote>#place_holder</blockquote><div class="attribution">-{}<br/><cite>{}</cite></div></div>"#,
                           block_id_str, block_title_str, author, source,
            )
        }
        BlockType::LiteralBlock => {
            return format!(r#"<div {} class="literalblock">{}<div class="content"><pre>#place_holder</pre></div></div>"#, block_id_str, block_title_str);
        }
        BlockType::ExampleBlock => {
            return format!(r#"<div {} class="exampleblock">{}<div class="content">#place_holder</div></div>"#, block_id_str, block_title_str);
        }
        BlockType::SidebarBlock => {
            return format!(r#"<div {} class="sidebarblock">{}<div class="content">#place_holder</div></div>"#, block_id_str, block_title_str);
        }
        // TODO, support id and title
        BlockType::AdmonitionBlock {marker} => {
            return format!(r#"<div class="admonitionblock {}"><table><tbody><tr><td class="icon"><div class="title">{}</div></td><td>#place_holder</td></tr></tbody></table></div>"#, marker, marker);
        }
        BlockType::NotBlock => {} // do nothing
    }

    // default tpl
    default_tpl
        .replace("#id_holder", block_id_str.as_str())
        .replace("#title_holder", block_title_str.as_str())
}

// verse_block_element = { verse_file_include | blank_line | verse_block_paragraph }
pub fn verse_block_element(ast: Pair<Rule>) -> String {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            // TODO
            // FIXME
            /*
            Rule::verse_file_include => verse_file_include(e),
            Rule::blank_line => blank_line(e),
            Rule::verse_block_paragraph => verse_block_paragraph(e),
            _ => unreachable!(),
            */
            _ => return e.as_str().to_string(),
        }
    }
    String::new()
}

pub fn verse_file_include(ast: Pair<Rule>) {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            Rule::file_inclusion => {
                file_inclusion(e);
            }
            _ => unreachable!(),
        }
    }
}

// verse_block_paragraph = { verse_block_paragraph_line+ }
pub fn verse_block_paragraph(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::verse_block_paragraph_line => verse_block_paragraph_line(e),
            _ => unreachable!(),
        }
    }
}

pub fn verse_block_paragraph_line(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::verse_block_paragraph_line_element => verse_block_paragraph_line_element(e),
            _ => unreachable!(),
        }
    }
}

pub fn verse_block_paragraph_line_element(ast: Pair<Rule>) {
    //println!("{:?}", ast);
    if let Some(e) = ast.into_inner().next() {
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
}

pub fn literal_block(ast: Pair<Rule>) -> String {
    if let Some(e) = ast.into_inner().next() {
        match e.as_rule() {
            Rule::paragraph_with_headingspaces => return paragraph_with_headingspaces(e),
            Rule::paragraph_with_literal_block_delimiter => {
                return paragraph_with_literal_block_delimiter(e)
            }
            _ => unreachable!(),
        }
    }
    String::new()
}

pub fn image_block(ast: Pair<Rule>) -> String {
    // url - alt - title
    let mut img = (String::new(), String::new(), String::new());
    for elem in ast.into_inner() {
        match elem.as_rule() {
            Rule::element_attributes => {
                // TODO
                // currently not supported
                let _b = element_attributes(elem);
                // TODO
            }
            Rule::URL => {
                img.0 = elem.as_str().to_string();
            }
            Rule::image_attributes => {
                img.1 = elem.as_str().to_string();
            }
            _ => unreachable!(),
        }
    }

    // TODO trim the suffix
    if img.1.is_empty() {
        img.1 = img
            .0
            .clone()
            .split(".")
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .to_string();
    }
    format!(
        r#"<div class="imageblock"><div class="content"><img src="{}" alt="{}"></div></div>"#,
        img.0, img.1
    )
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
