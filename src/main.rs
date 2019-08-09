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
use std::num::ParseIntError;
use std::hint::unreachable_unchecked;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ExprParser;

/// error occurred when parsing user input
#[derive(Debug)]
pub struct ParseError {
    pub location: pest::error::InputLocation,
    pub expected: String,
}

pub fn convert(query: String) {
    let x = query.as_str();
    let parse_result = ExprParser::parse(Rule::pre_flight_document, x);
    match parse_result {
        Ok(mut top_ast) => walk_tree(top_ast.next().unwrap()),
        Err(e) => println!("{:?}", e),
    }
}

pub fn document_blocks(elem: Pair<Rule>) {
    println!("document blocks");
    for i in elem.into_inner() {
        match i.as_rule() {
            Rule::document_header => {
                document_header(i.into_inner().next().unwrap());
            }
            Rule::document_block => {
                document_block(i.into_inner().next().unwrap());
            }
            _ => println!("skip in document block"),
        }
    }
}

// element_attributes = { element_attribute+ }
pub fn element_attributes(ast: Pair<Rule>) {
    let elems = ast.into_inner();
    for e in elems {
        match e.as_rule() {
            Rule::element_attribute => println!("elem attr"),
            _ => unreachable!(),
        }
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
pub fn section(ast: Pair<Rule>) {
    let mut c = ast.into_inner();
    for elem in c {
        match elem.as_rule() {
            Rule::element_attributes => println!("section : elem attr"),
            Rule::title_elements => println!("section : title elems"),
            Rule::inline_element_id => println!("section : inline elem id"),
            _ => unreachable!(),
        }
    }
}

pub fn delimited_block(ast: Pair<Rule>) {
    let mut elem = ast.into_inner().next().unwrap();
    match elem.as_rule() {
        Rule::fenced_block => println!("del blo : fence"),
        Rule::listing_block => println!("del blo : listing blo"),
        Rule::example_block => println!("del blo : examp blo"),
        Rule::verse_block => println!("del blo : verse blo"),
        Rule::quote_block => println!("del blo : quo blo"),
        Rule::sidebar_block => println!("del blo : sidebar blo"),
        Rule::single_line_comment => println!("del blo : sing lin cmt"),
        Rule::table => println!("del blo : tab"),
        Rule::comment_block => println!("del blo : cmt blo"),
        _ => unreachable!(),
    }
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
            Rule::element_attributes => println!("verse para : elem attr"),
            Rule::admonition_kind => println!("verse para : admo kind"),
            Rule::inline_elements => println!("verse para : inline elem"),
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
            Rule::element_attributes => println!("para : ela"),
            Rule::admonition_kind => println!("para : ak"),
            Rule::inline_elements => println!("para : ie"),
            _ => unreachable!(),
        }
    }
}

pub fn document_block(elem: Pair<Rule>) {
    match elem.as_rule() {
        Rule::simple_paragraph => simple_paragraph(elem),
        Rule::section => section(elem),
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


pub fn pre_flight_document(ast: Pair<Rule>) {
    let mut c = ast.into_inner();
    for elem in c {
        match elem.as_rule() {
            //~ front_matter*
            Rule::front_matter => {
                println!("front matter");
            }
            //~ document_block
            Rule::document_blocks => {
                document_blocks(elem);
            }
            _ => println!("skip"),
        }
    }
}

pub fn walk_tree(ast: Pair<Rule>) {
    println!("top match");
    match ast.as_rule() {
        Rule::pre_flight_document => {
            pre_flight_document(ast);
        }
        _ => unreachable!(),
    }
}

fn main() {
    // add toc to str will destroy the ast
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

image::trie.png[]

* 单个节点代表一个字母
* 如果需要对字符串进行匹配
* 只要从根节点开始依次匹配即可

1. first level list
2. first level list
3. first level list
4. first level list

[TIP]
Use abc to do some thing
to know the limit

limitw


[TIP]
====
dancing with eyes
====

[quote,Rūmī]
____
Patience is the key to joy.
____


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


.Possible DefOps manual locations
* West wood maze
** Maze heart
*** Reflection pool
** Secret exit
* Untracked file in git repository


.After landing the cloaked Klingon bird of prey in Golden Gate park:
[quote, Captain James T. Kirk, Star Trek IV: The Voyage Home]
Everybody remember where we parked.

[verse, Carl Sandburg, Fog]
____
The fog comes
on little cat feet.

It sits looking
over harbor and city
on silent haunches
and then moves on.
____


"#
    .to_string();
    convert(str);
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

*/

// 其它的 limitation 可以了解这里
// https://github.com/bytesparadise/libasciidoc/blob/master/LIMITATIONS.adoc
