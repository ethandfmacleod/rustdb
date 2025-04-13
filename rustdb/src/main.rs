use easy_repl::Repl;
mod commands;

fn main() {
    let mut repl = Repl::builder()
        .add("add", commands::add_command())
        .build()
        .expect("Failed to create repl");

    repl.run().expect("Critical REPL error");
}
