use simplelog::*;
use std::fs::File;

pub fn initialize_global_loggers() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed).unwrap(),
            WriteLogger::new(LevelFilter::max(), Config::default(), File::create("my_rust_binary\
            .log").unwrap()),
        ]
    ).unwrap();
}