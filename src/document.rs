use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let file_contents = fs::read_to_string(filename)?;

        let mut rows: Vec<Row> = Vec::new();
        for value in file_contents.lines() {
            rows.push(Row::from(value));
        }

        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
        })
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
}
