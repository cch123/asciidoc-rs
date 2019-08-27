use crate::structs::BlockType::{AdmonitionBlock, ExampleBlock, NotBlock};
use crate::structs::*;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExprParser;

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
            Rule::list_items => list_items(e),
            Rule::paragraph => {
                result += paragraph(e).as_str();
            }
            _ => unreachable!(),
        }
    }
    result
}

pub fn list_items(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::list_item => list_item(e),
            _ => unreachable!(),
        }
    }
}

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
                let e = elem.into_inner().next().unwrap();
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
    for elem in ast.into_inner() {
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
        level,
        title: result,
    }
}

pub fn delimited_block(ast: Pair<Rule>) -> String {
    let mut result = String::new();

    let elem = ast.into_inner().next().unwrap();
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
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        /*
        Rule::blank_line
        Rule::file_inclusion
        Rule::list_item
        Rule::fenced_block_paragraph
        */
        _ => e.as_str().to_string(),
    }
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

fn get_listing_block_tpl(block: Block, default_tpl: String) -> String {
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
            Rule::file_inclusion => file_inclusion(e),
            Rule::list_item => list_item(e),
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

// verse_block_element = { verse_file_include | blank_line | verse_block_paragraph }
pub fn verse_block_element(ast: Pair<Rule>) -> String {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        // TODO
        // FIXME
        /*
        Rule::verse_file_include => verse_file_include(e),
        Rule::blank_line => blank_line(e),
        Rule::verse_block_paragraph => verse_block_paragraph(e),
        _ => unreachable!(),
        */
        _ => e.as_str().to_string(),
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
pub fn document_attribute_substitution(_ast: Pair<Rule>) {
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
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::URL => println!("url"),
            Rule::image_attributes => println!("image attr"),
            _ => unreachable!(),
        }
    }
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
    let e = ast.clone().into_inner().next().unwrap();
    match e.as_rule() {
        Rule::blank_line => blank_line(e),
        // TODO
        Rule::file_inclusion => file_inclusion(e),
        // TODO
        Rule::list_item => list_item(e),
        Rule::non_sidebar_block => result += non_sidebar_block(e).as_str(),
        Rule::sidebar_block_paragraph => result += sidebar_block_paragraph(e).as_str(),
        _ => unreachable!(),
    }

    result
}

// non_sidebar_block = { !sidebar_block ~ delimited_block }
pub fn non_sidebar_block(ast: Pair<Rule>) -> String {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::delimited_block => {
            return delimited_block(e);
        }
        _ => unreachable!(),
    }
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
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::inline_elements => return inline_elements(e),
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

/*
pub fn comment_block(ast: Pair<Rule>) {
    for e in ast.into_inner() {
        match e.as_rule() {
            Rule::comment_block_delimiter => {} // do nothing
            Rule::NEWLINE => {}                 // do nothing?
            Rule::comment_block_line => comment_block_line(e),
            _ => unreachable!(),
        }
    }
}
*/

/*
pub fn comment_block_line(ast: Pair<Rule>) {
    // return string
}
*/

pub fn file_inclusion(ast: Pair<Rule>) {
    for e in ast.into_inner() {
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

pub fn image_block(ast: Pair<Rule>) -> String {
    // url - alt - title
    let mut img = (String::new(), String::new(), String::new());
    for elem in ast.into_inner() {
        match elem.as_rule() {
            Rule::element_attributes => {
                // TODO
                // currently not supported
                let b = element_attributes(elem);
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
        img.1 = img.0.clone()
    }
    format!(
        r#"<div class="imageblock"><div class="content"><img src="{}" alt="{}"></div></div>"#,
        img.0, img.1
    )
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

pub fn blank_line(_ast: Pair<Rule>) {
    // do nothing
}

pub fn literal_block(ast: Pair<Rule>) -> String {
    let e = ast.into_inner().next().unwrap();
    match e.as_rule() {
        Rule::paragraph_with_headingspaces => paragraph_with_headingspaces(e),
        Rule::paragraph_with_literal_block_delimiter => paragraph_with_literal_block_delimiter(e),
        _ => unreachable!(),
    }
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
}

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

pub fn front_matter(ast: Pair<Rule>) {
    let elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::yaml_front_matter => yaml_front_matter(elem),
        _ => unreachable!(),
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
