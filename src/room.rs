use crate::interaction::command;

pub mod room {
    struct Room {
        room_id: String,
        description: String,
        commands: Vec<Command>,
    }
}
