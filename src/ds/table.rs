#[derive(Debug)]
pub struct Table {
    pub cols: Vec<Column>,
}

#[derive(Debug)]
pub struct TableWithMeta {
    pub meta: Vec<String>,
    pub special_case: SpecialCase,
    pub table: Table,
}

impl Table {
    pub fn new(cols: Vec<Column>) -> Self {
        Table { cols }
    }
}

impl TableWithMeta {
    pub fn new(meta: Vec<String>, table: Table) -> Self {
        TableWithMeta { meta, special_case: SpecialCase::None, table }
    }

    pub fn with_special_case(special_case: SpecialCase, table: Table) -> Self {
        TableWithMeta { meta: Vec::<String>::new(), special_case, table}
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SpecialCase {
    None,
    SpaceAndHighlightOn4
}

#[derive(Debug)]
pub struct Column {
    pub header: Header,
    pub contents: Vec<String>,
    pub footer: Option<String>,
}

impl Column {
    pub fn new(header: Header, footer: Option<String>, capacity: usize) -> Self {
        Column {
            header,
            contents: Vec::<String>::with_capacity(capacity),
            footer,
        }
    }

    pub fn from_contents(header: Header, contents: Vec<String>, footer: Option<String>) -> Self {
        Column {
            header,
            contents,
            footer
        }
    }
}

#[derive(Debug)]
pub struct Header {
    pub text: String,
    pub highlight: bool,
}

impl Header {
    pub fn new(text: String, highlight: bool) -> Self {
        Header { text, highlight }
    }
}
