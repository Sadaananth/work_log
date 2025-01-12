pub mod database {
    use crate::common::common::Entry;
    use chrono::{TimeZone, Utc};
    use colour::*;
    use dirs;
    use duckdb::{params, Connection};

    pub struct DatabaseHandler {
        conn: Connection,
    }

    impl DatabaseHandler {
        pub fn new(use_in_memory: bool) -> Self {
            DatabaseHandler {
                conn: if use_in_memory {
                    Connection::open_in_memory()
                        .expect("Failed to open connection to in memory database")
                } else {
                    let home_dir = dirs::home_dir().expect("Failed to get home directory");
                    let db_path = home_dir.join(".worklog.db");
                    Connection::open(db_path).expect("Failed to open connection to database")
                },
            }
        }
        pub fn init_handler(&self) {
            self.conn
                .execute_batch(
                    r"CREATE TABLE IF NOT EXISTS worklog (
                  date           INTEGER PRIMARY KEY,
                  entry          INTEGER,
                  exit           INTEGER
                  );
         ",
                )
                .expect("Failed to create table");
        }
        pub fn add_entry(&self, midnight: u64, time: u64) {
            self.conn
                .execute(
                    "INSERT INTO worklog (date, entry, exit) VALUES (?, ?, ?)  ON CONFLICT DO UPDATE SET entry = EXCLUDED.entry;",
                    params![midnight, time, 0u64],
                )
                .expect("Failed to insert worklog entry");
            println!("Added entry. Good luck on staring ur day fresh");
        }
        pub fn add_exit(&self, midnight: u64, time: u64) {
            self.conn
                .execute(
                    "INSERT INTO worklog (date, entry, exit) VALUES (?, ?, ?) ON CONFLICT DO UPDATE SET exit = EXCLUDED.exit;",
                    params![midnight, 0u64, time],
                )
                .expect("Failed to insert worklog exit");
            println!("Added exit. Enjoy your leave");
        }

        pub fn pretty_print(&self, midnight: u64, entry: u64, exit: u64) {
            let midnight_utc = Utc.timestamp_opt(midnight.try_into().unwrap(), 0).unwrap();
            let midnight_local = midnight_utc.with_timezone(&chrono::Local);
            let entry_utc = Utc.timestamp_opt(entry.try_into().unwrap(), 0).unwrap();
            let entry_local = entry_utc.with_timezone(&chrono::Local);
            let exit_utc = Utc.timestamp_opt(exit.try_into().unwrap(), 0).unwrap();
            let exit_local = exit_utc.with_timezone(&chrono::Local);
            yellow_ln_bold!("{}", midnight_local);
            println!("Entry: {}", entry_local);
            println!("Exit: {}", exit_local);
            println!("\n\tNo work logged for this day :)\n");
        }

        pub fn print(&self) {
            let mut stmt = self
                .conn
                .prepare("SELECT date, entry, exit FROM worklog")
                .expect("Failed to get data");
            let log_iter = stmt
                .query_map([], |row| {
                    Ok(Entry {
                        date: row.get(0)?,
                        entry: row.get(1)?,
                        exit: row.get(2)?,
                    })
                })
                .expect("Query failed");

            for log in log_iter {
                let value = log.unwrap();
                self.pretty_print(value.date, value.entry, value.exit)
            }
        }

        pub fn get_entry(&self) -> Vec<Entry> {
            let mut stmt = self
                .conn
                .prepare("SELECT date, entry, exit FROM worklog")
                .expect("Failed to get data");
            let log_iter = stmt
                .query_map([], |row| {
                    Ok(Entry {
                        date: row.get(0)?,
                        entry: row.get(1)?,
                        exit: row.get(2)?,
                    })
                })
                .expect("Query failed");

            let mut data = Vec::new();
            for row in log_iter {
                data.push(row.unwrap());
            }
            data
        }
    }
}
