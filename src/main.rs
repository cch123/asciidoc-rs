#![recursion_limit = "1024"]

extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate serde_json;
//use pest::prec_climber::{Assoc, Operator, PrecClimber};

//use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ExprParser;

/// error occurred when parsing user input
#[derive(Debug)]
pub struct ParseError {
    pub location: pest::error::InputLocation,
    pub expected: String,
}

pub fn convert(
    query: String
) {
    let x = query.as_str();
    let parse_result = ExprParser::parse(Rule::pre_flight_document, x);
    println!("{:#?}", parse_result);
}

//use pest::iterators::Pair;

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

.After landing the cloaked Klingon bird of prey in Golden Gate park:
[quote, Captain James T. Kirk, Star Trek IV: The Voyage Home]
Everybody remember where we parked.

[, James Baldwin]
""
Not everything that is faced can be changed.
But nothing can be changed until it is faced.
""


[verse, Carl Sandburg, Fog]
____
The fog comes
on little cat feet.

It sits looking
over harbor and city
on silent haunches
and then moves on.
____


"#.to_string();
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

// 其它的 limitation 可以了解这里
// https://github.com/bytesparadise/libasciidoc/blob/master/LIMITATIONS.adoc
