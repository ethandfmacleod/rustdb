#[derive(Debug)]
pub enum ColumnType {
    String,
    Float32,
    Integer32,
    Boolean
}

#[derive(Debug, Clone)]
pub enum RowValue {
    String(String),
    Float32(f32),
    Integer32(i32),
    Boolean(bool),
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<(String, ColumnType)>,
    pub rows: Vec<Vec<RowValue>>
}