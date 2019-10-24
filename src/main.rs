use std::fs::File;
use std::process::exit;

use log::{debug, error, info, trace, warn};
use simplelog::*;

use crate::user_interface::ui::{read_command, tell};
use crate::interaction::command::{Command};
use crate::interaction::look_command::LookCommand;

mod user_interface;
mod interaction;

fn main() {
    initialize_global_loggers();

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
