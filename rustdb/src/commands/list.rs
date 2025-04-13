use easy_repl::{command, CommandStatus, Command};
use std::sync::{Arc, Mutex};
use crate::db::Database;

pub fn list_command(db: Arc<Mutex<Database>>) -> Command<'static> {
    let db_clone = db.clone();

    command! {
        "List objects: list <table|schema|rows> [table_name]",
        (object_type: String, table_name: String) => { let db = db_clone.lock().unwrap(); move |object_type: String, table_name: String| {
            let maybe_name = if table_name.trim().is_empty() {
                None
            } else {
                Some(table_name)
            };

            match object_type.trim().to_lowercase().as_str() {
                "table" => {
                    println!("Tables:");
                    for name in db.tables.keys() {
                        println!("- {}", name);
                    }
                },
                "schema" => {
                    if let Some(table_name) = maybe_name {
                        match db.tables.get(&table_name) {
                            Some(table) => {
                                println!("Schema for '{}':", table.name);
                                for (col_name, col_type) in &table.columns {
                                    println!("  - {}: {:?}", col_name, col_type);
                                }
                            },
                            None => println!("Table '{}' not found", table_name),
                        }
                    } else {
                        println!("Usage: list schema <table_name>");
                    }
                },
                "rows" => {
                    if let Some(table_name) = maybe_name {
                        match db.tables.get(&table_name) {
                            Some(table) => {
                                println!("Rows in '{}':", table.name);
                                for (i, row) in table.rows.iter().enumerate() {
                                    println!("  Row {}:", i + 1);
                                    for (col, value) in table.columns.iter().zip(row.iter()) {
                                        println!("    {} = {:?}", col.0, value);
                                    }
                                }
                            },
                            None => println!("Table '{}' not found", table_name),
                        }
                    } else {
                        println!("Usage: list rows <table_name>");
                    }
                },
                _ => {
                    println!("Unknown list type: '{}'", object_type);
                    println!("Use: list table | list schema <table> | list rows <table>");
                }
            }

            Ok(CommandStatus::Done)
        }}
    }
}
