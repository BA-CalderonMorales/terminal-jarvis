//! ValueBlock shapes: the data a card is made of -- groups of label/value
//! rows. The rendering lives in `cli::logic::value_block`; these are the
//! shapes that cross into it.

/// One row of a group: a label and the value under it.
pub struct Row {
    pub label: String,
    pub value: String,
}

impl Row {
    pub fn new(label: &str, value: impl Into<String>) -> Self {
        Self {
            label: label.to_string(),
            value: value.into(),
        }
    }
}

/// One group of a card: a heading and the rows beneath it.
pub struct Group {
    pub heading: String,
    pub rows: Vec<Row>,
}

impl Group {
    pub fn new(heading: &str, rows: Vec<Row>) -> Self {
        Self {
            heading: heading.to_string(),
            rows,
        }
    }
}
