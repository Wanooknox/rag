
use std::process::exit;

use log::{debug, error, info, trace, warn};

use crate::boot::boot::boot;
use crate::interaction::command::Command;
use crate::interaction::look_command::LookCommand;
use crate::loading::room_reader::{read_rooms};
use crate::user_interface::ui::{read_command, tell};

mod user_interface;
mod interaction;
mod loading;
mod boot;

fn main() {
    boot();

    let rooms = read_rooms("resources/rooms.json".to_string());
    println!("{}", rooms);

    println!("Hello, world!");
    info!("Hello Info");
    trace!("Hello Trace");
    warn!("Hello Warning!");
    error!("Hello ERROR!");
    debug!("Hello Debug!");

    loop {
        tell("Write your command".to_string());
        let command = read_command();
        process_command(command);
    }
}

fn process_command(command: String) {
    if command.as_str() == "quit" {
        exit(0);
    }
    tell(command);
}
