use ::work_log::worklog::WorkLog;
use work_log::plot::plot;

fn main() {
    let worklog = WorkLog::new(true);
    match worklog.add_entry_at("2023-10-01T12:34:56Z") {
        Ok(_) => println!("Entry added successfully."),
        Err(e) => println!("Failed to add entry: {}", e),
    }
    match worklog.add_entry_at("2023-10-01T13:34:56Z") {
        Ok(_) => println!("Exit added successfully."),
        Err(e) => println!("Failed to add entry: {}", e),
    }
    match worklog.add_entry_at("2023-10-02T14:34:56Z") {
        Ok(_) => println!("Entry added successfully."),
        Err(e) => println!("Failed to add entry: {}", e),
    }
    match worklog.add_entry_at("2023-10-02T15:34:56Z") {
        Ok(_) => println!("Exit added successfully."),
        Err(e) => println!("Failed to add entry: {}", e),
    }
    match worklog.add_entry_at("2023-10-03T02:34:56Z") {
        Ok(_) => println!("Entry added successfully."),
        Err(e) => println!("Failed to add entry: {}", e),
    }
    match worklog.add_entry_at("2023-10-03T03:34:56Z") {
        Ok(_) => println!("Exit added successfully."),
        Err(e) => println!("Failed to add entry: {}", e),
    }
    worklog.print();
    let entry = worklog.database_handler.get_entry();
    plot::plot_entries(entry);
}
