use easy_repl::{command, CommandStatus, Command};
use std::sync::{Arc, Mutex};

use crate::db::Database;
use crate::table::{Table, ColumnType};

pub fn create_table_command(db: Arc<Mutex<Database>>) -> Command<'static> {
    command! {
        "Create a table: create_table <table_name> <col1:type1,col2:type2,...>",
        (input: String) => {
            let db = db.clone(); 
            move |input: String| {
                let parts: Vec<&str> = input.splitn(2, ' ').collect();
                if parts.len() < 2 {
                    println!("Usage: create_table <table_name> <col1:type1,col2:type2,...>");
                    return Ok(CommandStatus::Done);
                }
                
                let table_name = parts[0].to_string();
                
                // Basic table name validation
                if table_name.is_empty() {
                    println!("Table name cannot be empty.");
                    return Ok(CommandStatus::Done);
                }

                let column_defs = parts[1].to_string();
                let mut columns: Vec<(String, ColumnType)> = Vec::new();

                for col_def in column_defs.split(',') {
                    let parts: Vec<&str> = col_def.trim().split(':').collect();
                    if parts.len() != 2 {
                        println!("Invalid column definition: '{}'. Format should be name:type", col_def);
                        return Ok(CommandStatus::Done);
                    }
                    let col_name = parts[0].trim().to_string();
                    
                    // Basic column name validation
                    if col_name.is_empty() {
                        println!("Column name cannot be empty.");
                        return Ok(CommandStatus::Done);
                    }

                    let col_type = match parts[1].trim().to_lowercase().as_str() {
                        "string" => ColumnType::String,
                        "float32" => ColumnType::Float32,
                        "integer32" => ColumnType::Integer32,
                        "boolean" => ColumnType::Boolean,
                        unknown => {
                            println!("Unknown column type: '{}'. Supported types: string, float32, integer32, boolean", unknown);
                            return Ok(CommandStatus::Done);
                        }
                    };
                    columns.push((col_name, col_type));
                }

                if columns.is_empty() {
                    println!("Table must have at least one column.");
                    return Ok(CommandStatus::Done);
                }

                let mut db = db.lock().unwrap();
                if db.tables.contains_key(&table_name) {
                    println!("Table '{}' already exists.", table_name);
                } else {
                    let column_count = columns.len();
                    let table = Table {
                        name: table_name.clone(),
                        columns,
                        rows: Vec::new(),
                    };
                    db.tables.insert(table_name.clone(), table);
                    println!("Table '{}' created successfully with {} column(s).", table_name, column_count);
                }

                Ok(CommandStatus::Done)
            }
        }
    }
}