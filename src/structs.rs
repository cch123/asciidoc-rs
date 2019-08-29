/// error occurred when parsing user input
#[derive(Debug)]
pub struct ParseError {
    pub location: pest::error::InputLocation,
    pub expected: String,
}

#[derive(Debug)]
pub struct Block {
    pub id: String,
    pub role: String,
    pub title: String,
    pub block_type: BlockType,
}

#[derive(Debug, Clone)]
pub enum BlockType {
    NotBlock,
    LiteralBlock,
    ExampleBlock,
    SidebarBlock,
    AdmonitionBlock { marker: String },
    VerseBlock { author: String, source: String },
    QuoteBlock { author: String, source: String },
    SourceBlock { lang: String },
}

pub struct SectionHeader {
    pub level: usize,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub level: usize,
    pub content: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ListItemType {
    OrderedItem,
    UnorderedItem,
    LabeledItem,
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub typ: ListItemType,
    pub level: i8,
    pub children: Vec<ListItem>,
    pub title: String,
    pub description: String, // only labeled item has a description
}

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        return other.typ == self.typ && other.level == self.level;
    }
}
