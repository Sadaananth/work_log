use::work_log::worklog::WorkLog;
use work_log::plot::plot;

fn main() {
    let worklog = WorkLog::new();
    worklog.add_entry();
    worklog.add_exit();
    let entry = worklog.database_handler.get_entry();
    plot::plot_entries(entry);
}
