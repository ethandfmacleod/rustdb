use easy_repl::{command, CommandStatus};

pub fn add_command() -> easy_repl::Command<'static> {
    command! {
        "Add X to Y",
        (x: i32, y: i32) => |x, y| {
            println!("{} + {} = {}", x, y, x + y);
            Ok(CommandStatus::Done)
        }
    }
}
