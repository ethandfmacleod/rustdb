use std::{collections::HashMap, sync::{Arc, Mutex}};

use db::Database;
use easy_repl::Repl;
mod db;
mod table;
mod commands;

fn main() {
    let db = Arc::new(Mutex::new(Database { tables: HashMap::new() }));
    let mut repl = Repl::builder()
        .add("add", commands::add_command())
        .add("insert", commands::insert_command())
        .add("create_table", commands::create_table_command(db))
        .build()
        .expect("Failed to create repl");

    repl.run().expect("Critical REPL error");
}
