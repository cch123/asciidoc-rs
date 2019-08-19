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

#[derive(Debug)]
pub enum BlockType {
    NotBlock,
    LiteralBlock,
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
