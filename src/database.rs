pub mod database {
    use duckdb::{params, Connection};

    pub struct DatabaseHandler {
        conn: Connection,
    }

    impl DatabaseHandler {
        pub fn new() -> Self {
            DatabaseHandler {
                conn: Connection::open_in_memory().expect("Failed to open connection"),
            }
        }
        pub fn init_handler(&self) {
            self.conn
                .execute_batch(
                    r"CREATE SEQUENCE seq;
          CREATE TABLE salary (
                  id             INTEGER PRIMARY KEY DEFAULT NEXTVAL('seq'),
                  date           TEXT NOT NULL,
                  entry          TEXT,
                  exit           TEXT
                  );
         ",
                )
                .expect("Failed to create table");
        }
    }
}
