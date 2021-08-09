use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>
}

impl Document {
    pub fn open() -> Self {
        let mut rows: Vec<Row> = Vec::new();
        rows.push(Row::from("Hello world!"));
        Self {
            rows
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
}