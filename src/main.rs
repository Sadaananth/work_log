use clap::Parser;
use::work_log::worklog::WorkLog;

#[derive(Parser)]
struct Cli {
    command: Option<String>,
}

fn main() {
    let work_log = WorkLog::new();
    let args = Cli::parse();

    match args.command {
        None => {}
        Some(value) => match value.as_str() {
            "entry" => work_log.add_entry(),
            "exit" => work_log.add_exit(),
            "show" => work_log.print(),
            _ => work_log.print()
        },
    }
}
