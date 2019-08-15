/*
// Document Root
pub enum Doc {
    PreFlightDocument {
        front_matter: Option<FrontMatter>,
        document_blocks: DocumentBlocks,
    },
    PreFlightDocumentWithinDelimitedBlock {
        front_matter: Option<FrontMatter>,
        document_blocks_within_delimited_block: DocumentBlocksWithinDelimitedBlock,
    },
}

pub struct DocumentBlocksWithinDelimitedBlock {
    document_blocks_within_delimited_block: Vec<DocumentBlocksWithinDelimitedBlock>,
}

pub struct DocumentBlockWithinDelimitedBlock {
    content: String,
}

pub struct DocumentBlocks {
    document_header: Option<DocumentHeader>,
    document_blocks: Vec<DocumentBlock>,
}

pub struct DocumentHeader {
    content: String,
}

pub enum DocumentBlock {
    SimpleParagraph {
        element_attributes: Option<ElementAttributes>,
        first_paragraph_line: String,
        other_paragraph_line: Vec<InlineElements>,
    },
    Section(String),
    DelimitedBlock(String),
    FileInclusion(String),
    VerseParagraph(String),
    ImageBlock(String),
    ListItem(String),
    BlankLine(String),
    LiteralBlock(String),
    DocumentAttributeDeclaration(String),
    DocumentAttributeReset(String),
    TableOfContentsMacro(String),
    UserMacroBlock(String),
    Paragraph(String),
}

// Rust 枚举就是牛逼
pub enum InlineElements {
    InlineElements { inline_elements: Vec<InlineElement> },
    SingleLineComment(String),
}

pub enum InlineElement {
    SimpleWord(String),
    Spaces(String),
    InlineImage(String),
    Link(String),
    PassThrough(String),
    InlineFootnote(String),
    InlineUserMacro(String),
    QuotedText(String),
    CrossReference(String),
    DocumentAttributeSubstitution(String),
    InlineElementId(String),
    OtherWord(String),
}

pub struct ElementAttributes {
    element_attributes: Vec<ElementAttribute>,
}

pub struct ElementAttribute(String);

pub struct FrontMatter {
    yaml_front_matter: YamlFrontMatter,
}

pub struct YamlFrontMatter {
    yaml_front_matter_token: String,
    yaml_front_matter_content: Option<String>,
}

*/
