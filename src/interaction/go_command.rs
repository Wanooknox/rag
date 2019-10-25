use crate::interaction::command::Command;

pub struct GoCommand {
    pub north_destination: String,
    pub east_destination: String,
    pub south_destination: String,
    pub west_destination: String,
}

impl Command for GoCommand {
    fn execute(&self) {
        // TODO: handle go
    }
}