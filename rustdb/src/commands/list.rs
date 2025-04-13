use easy_repl::{command, CommandStatus};

pub fn list_command() -> easy_repl::Command<'static> {
    command! {
        "List DB Objects, Table or rows",
        (object: String) => |object| {
            println!("Inserted into table value: {}", value);
            Ok(CommandStatus::Done)
        }
    }
}
