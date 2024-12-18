use clap::Parser;
pub mod database;
use database::database::DatabaseHandler;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
struct Cli {
    command: Option<String>,
}

fn get_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

fn get_midnight(time: u64) -> u64 {
    let seconds_in_day = 24 * 60 * 60;
    let secs_from_midnight = time % seconds_in_day;
    time - secs_from_midnight
}

fn add_entry(handler: &DatabaseHandler) {
    let time = get_time();
    handler.add_entry(get_midnight(time), time);
}

fn add_exit(handler: &DatabaseHandler) {
    let time = get_time();
    handler.add_exit(get_midnight(time), time);
}

fn main() {
    let database_handler = DatabaseHandler::new();
    database_handler.init_handler();
    let args = Cli::parse();

    match args.command {
        None => {}
        Some(value) => match value.as_str() {
            "entry" => add_entry(&database_handler),
            "exit" => add_exit(&database_handler),
            "print" => database_handler.print(),
            _ => {}
        },
    }
    database_handler.print();
}
