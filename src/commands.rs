pub mod commands {
    use crate::interaction::command::Command;
    use crate::interaction::go_command::GoCommand;

    pub struct CommandBuilder {}

    impl CommandBuilder {
        pub fn with(command: String, arg_list: Vec<String>) -> Result<Box<dyn Command>,()> {
            if command == "go".to_string() {
                return Ok(Box::new(GoCommand {
                    north_destination: String::from("north"),
                    south_destination: String::from("south"),
                    east_destination: String::from("east"),
                    west_destination: String::from("west")
                }));
            }
            print!("I AM SCREAMING!");
            return Err(());
        }
    }
}