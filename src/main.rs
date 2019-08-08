#![recursion_limit = "1024"]
pub mod ast;

extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate serde_json;
//use pest::prec_climber::{Assoc, Operator, PrecClimber};

//use pest::error::Error;
use pest::Parser;
use pest::iterators::Pair;
use std::num::ParseIntError;

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
        Ok(mut top_ast) => pre_flight_document(top_ast.next().unwrap()),
        Err(e) => println!("{:?}", e),
    }
}

pub fn document_blocks(elem : Pair<Rule>) {
    println!("document blocks");
    for i in elem.into_inner() {
        match i.as_rule() {
            Rule::document_header => {
                document_header(i.into_inner().next().unwrap());
            },
            Rule::document_block => {
                document_block(i.into_inner().next().unwrap());
            },
            _ => println!("skip in document block"),
        }
    }
}

pub fn simple_paragraph(ast: Pair<Rule>) {
    let mut c = ast.into_inner();
    for elem in c {
        match elem.as_rule() {
            Rule::element_attributes => println!("simple para : elem attr"),
            Rule::first_paragraph_line => println!("simple para : first line"),
            Rule::other_paragraph_line => println!("simple para : other line"),
            _ => unreachable!(),
        }
    }
}

pub fn document_header(elem: Pair<Rule>) {
    println!("document header");
}

/*
    element_attributes?
    ~ "="{1,6}
    ~ title_elements ~ inline_element_id* ~ EOL
*/
pub fn section(ast: Pair<Rule>) {
    println!("section {:?}", ast);
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
        Rule::fenced_block => {
            println!("del blo : fence")
        },
        Rule::listing_block => {
            println!("del blo : listing blo")
        },
        Rule::example_block => {
            println!("del blo : examp blo")
        },
        Rule::verse_block => {
            println!("del blo : verse blo")
        },
        Rule::quote_block => {
            println!("del blo : quo blo")
        },
        Rule::sidebar_block => {
            println!("del blo : sidebar blo")
        },
        Rule::single_line_comment => {
            println!("del blo : sing lin cmt")
        },
        Rule::table => {
            println!("del blo : tab")
        },
        Rule::comment_block => {
            println!("del blo : cmt blo")
        },
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
            },
            Rule::file_include_attributes => {
                println!("file inc : file inc attr");
            },
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
            Rule::element_attributes => {
                println!("verse para : elem attr")
            },
            Rule::admonition_kind => {
                println!("verse para : admo kind")
            },
            Rule::inline_elements => {
                println!("verse para : inline elem")
            },
            _ => unreachable!(),
        }
    }
}

pub fn document_block(elem: Pair<Rule>) {
    match elem.as_rule() {
        Rule::simple_paragraph => {
            simple_paragraph(elem);
        },
        Rule::section => {
            section(elem);
        },
        Rule::delimited_block => {
            delimited_block(elem);
        },
        Rule::file_inclusion => {
            file_inclusion(elem);
        },
        Rule::verse_paragraph => {
            verse_paragraph(elem);
        },
        Rule::image_block => {
            println!("image block")
        },
        Rule::list_item => {
            println!("list item")
        },
        Rule::blank_line => {
            println!("blank line")
        },
        Rule::literal_block => {
            println!("literal blo")
        },
        Rule::document_attribute_declaration => {
            println!("document attri decl")
        },
        Rule::document_attribute_reset => {
            println!("doc attr reset")
        },
        Rule::table_of_contents_macro => {
            println!("toc macro")
        },
        Rule::user_macro_block => {
            println!("user macro")
        },
        Rule::paragraph => {
            println!("para")
        },
        _ => unreachable!(),
    }
}

pub fn pre_flight_document(ast: Pair<Rule>) {
    // println!("{:#?}", ast);
    match ast.as_rule() {
        Rule::pre_flight_document => {
            println!("top match");
            let mut c = ast.into_inner();
            for elem in c {
                match elem.as_rule() {
                    //~ front_matter*
                    Rule::front_matter => {
                        println!("front matter");
                    },
                    //~ document_block
                    Rule::document_blocks => {
                        document_blocks(elem);
                    },
                    _ => println!("skip"),
                }
            }
        },
        _ =>  unreachable!(),
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
