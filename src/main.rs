use std::fs::File;
use std::process::exit;

use log::{debug, error, info, trace, warn};
use simplelog::*;

use crate::user_interface::ui::{read_command, tell};
use crate::interaction::command::{Command};
use crate::interaction::look_command::LookCommand;
use crate::loading::room_reader::{read_rooms};

mod user_interface;
mod interaction;
mod loading;

fn main() {
    initialize_global_loggers();

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

fn initialize_global_loggers() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed).unwrap(),
            WriteLogger::new(LevelFilter::max(), Config::default(), File::create("my_rust_binary\
            .log").unwrap()),
        ]
    ).unwrap();
}

fn process_command(command: String) {
    if command.as_str() == "quit" {
        exit(0);
    }
    tell(command);
}
