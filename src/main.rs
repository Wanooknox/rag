use std::fs::File;
use log::{debug, error, info, trace, warn};
use simplelog::*;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed).unwrap(),
            WriteLogger::new(LevelFilter::max(), Config::default(), File::create("my_rust_binary\
            .log").unwrap()),
        ]
    ).unwrap();

    println!("Hello, world!");
    info!("Hello Info");
    trace!("Hello Trace");
    warn!("Hello Warning!");
    error!("Hello ERROR!");
    debug!("Hello Debug!");
}
