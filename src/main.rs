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

"#.to_string();
    convert(str);
}