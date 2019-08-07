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
    SimpleParagraph { content: String },
    Section { content: String },
    DelimitedBlock { content: String },
    FileInclusion { content: String },
    VerseParagraph { content: String },
    ImageBlock { content: String },
    ListItem { content: String },
    BlankLine { content: String },
    LiteralBlock { content: String },
    DocumentAttributeDeclaration { content: String },
    DocumentAttributeReset { content: String },
    TableOfContentsMacro { content: String },
    UserMacroBlock { content: String },
    Paragraph { content: String },
}

pub struct FrontMatter {
    yaml_front_matter: YamlFrontMatter,
}

pub struct YamlFrontMatter {
    yaml_front_matter_token: String,
    yaml_front_matter_content: Option<String>,
}
