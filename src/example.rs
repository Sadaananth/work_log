use ::work_log::worklog::WorkLog;
use work_log::plot::plot;

fn main() {
    let worklog = WorkLog::new(true);
    match worklog.add_entry_at("2023-10-01T12:34:56+08:00") {
        Ok(_) => {},
        Err(e) => println!("Failed to add entry: {}", e),
    }
    match worklog.add_exit_at("2023-10-01T13:34:56+08:00") {
        Ok(_) => {},
        Err(e) => println!("Failed to add exit: {}", e),
    }
    match worklog.add_entry_at("2023-10-02T14:34:56+08:00") {
        Ok(_) => {},
        Err(e) => println!("Failed to add exit: {}", e),
    }
    match worklog.add_exit_at("2023-10-02T15:34:56+08:00") {
        Ok(_) => {},
        Err(e) => println!("Failed to add exit: {}", e),
    }
    match worklog.add_entry_at("2023-10-03T15:34:56+08:00") {
        Ok(_) => {},
        Err(e) => println!("Failed to add entry: {}", e),
    }
    match worklog.add_exit_at("2023-10-03T17:34:56+08:00") {
        Ok(_) => {},
        Err(e) => println!("Failed to add exit: {}", e),
    }
    worklog.print();
    let entry = worklog.database_handler.get_entry();
    plot::plot_entries(entry);
}
