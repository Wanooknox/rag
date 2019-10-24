use crate::interaction::command::Command;

pub struct LookCommand {
    pub look_text: String
}

impl LookCommand {
    pub fn new(text: String) -> LookCommand {
        LookCommand {
            look_text: text
        }
    }
}

impl Command for LookCommand {
    fn execute(&self) {
        println!("{}",self.look_text);
    }
}
