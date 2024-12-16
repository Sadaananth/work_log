use clap::Parser;
pub mod database;
use database::database::DatabaseHandler;

#[derive(Parser)]
struct Cli {
    entry: Option<String>,
    exit: Option<String>,
}

fn add_entry(value: String) {

}

fn add_exit(value:String) {

}

fn main() {
    let database_handler = DatabaseHandler::new();
    database_handler.init_handler();
    let args = Cli::parse();
    println!("{:?} {:?}", args.entry, args.exit);

    match args.entry {
        None => {},
        Some(value) => add_entry(value),
    }

    match args.exit {
        None => {},
        Some(value) => add_exit(value),
    }
}
