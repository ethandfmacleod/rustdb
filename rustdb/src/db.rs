use std::collections::HashMap;
use crate::table::Table;

#[derive(Debug, Default)]
pub struct Database {
    pub tables: HashMap<String, Table>,
}