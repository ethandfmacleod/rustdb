use easy_repl::{command, CommandStatus};

pub fn insert_command() -> easy_repl::Command<'static> {
    command! {
        "Insert row into table.",
        (value: i32) => |value| {
            println!("Inserted into table value: {}", value);
            Ok(CommandStatus::Done)
        }
    }
}
