pub mod common;
pub mod database;
pub mod plot;

pub mod worklog {
    use crate::database::database::DatabaseHandler;
    use std::time::{SystemTime, UNIX_EPOCH};
    use chrono::{DateTime, Local, ParseError};

    pub struct WorkLog {
        pub database_handler: DatabaseHandler,
    }

    impl WorkLog {
        pub fn new(use_in_memory: bool) -> Self {
            let database_handler = DatabaseHandler::new(use_in_memory);
            database_handler.init_handler();
            WorkLog { database_handler }
        }

        fn get_time(&self) -> u64 {
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            since_the_epoch.as_secs()
        }

        fn get_midnight(&self, time: u64) -> u64 {
            let seconds_in_day = 24 * 60 * 60;
            let secs_from_midnight = time % seconds_in_day;
            println!("time {}, secs_from_midnight {}", time, secs_from_midnight);
            time - secs_from_midnight
        }

        pub fn add_entry(&self) {
            let time = self.get_time();
            self.database_handler
                .add_entry(self.get_midnight(time), time);
        }

        pub fn add_entry_at(&self, date_str: &str) -> Result<(), ParseError> {
            let datetime = DateTime::parse_from_rfc3339(date_str)?;
            let local_datetime = datetime.with_timezone(&Local);
            let timestamp = local_datetime.timestamp() as u64;
            self.database_handler
                .add_entry(self.get_midnight(timestamp), timestamp);
            Ok(())
        }

        pub fn add_exit(&self) {
            let time = self.get_time();
            self.database_handler
                .add_exit(self.get_midnight(time), time);
        }

        pub fn add_exit_at(&self, date_str: &str) -> Result<(), ParseError> {
            let datetime = DateTime::parse_from_rfc3339(date_str)?;
            let local_datetime = datetime.with_timezone(&Local);
            let timestamp = local_datetime.timestamp() as u64;
            self.database_handler
                .add_exit(self.get_midnight(timestamp), timestamp);
            Ok(())
        }

        pub fn print(&self) {
            self.database_handler.print();
        }
        pub fn plot(&self) {
            let entries = self.database_handler.get_entry();
            crate::plot::plot::plot_entries(entries);
        }
    }
}
