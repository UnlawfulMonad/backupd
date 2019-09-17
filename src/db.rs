use std::path::Path;
use rusqlite::Connection;
use crate::error as e;

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new<P: AsRef<Path>>(path: P, key: &str) -> e::Result<DB> {
        assert!(key.len() > 0);

        let conn = Connection::open(path)?;
        conn.pragma_update(None, "key", &key)?;
        Ok(DB { conn })
    }

    pub fn run_migrations(&mut self) -> e::Result<()> {
        const MIGRATIONS_TABLE: &str = "CREATE TABLE migrations (
                name TEXT NOT NULL,
                run_successfully BOOL
            )";
        if !self.table_exists("migrations") {
            self.conn.execute_batch(MIGRATIONS_TABLE)?;
        }
        let tx = self.conn.transaction()?;

        tx.commit()?;
        Ok(())
    }

    fn table_exists(&self, name: &str) -> bool {
        //let result: Result<String, _> = self.conn.query_row("SELECT name FROM pragma_table_info(?)", &[name], |row| row.get(0));
        let mut ret = false;
        self.conn.pragma(None, "table_info", &name, |_row| {
            ret = true;
            Ok(())
        }).unwrap();
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn open_new_db() {
        let mut db = DB::new("test.db", "test").unwrap();
        assert!(!db.table_exists("migrations"));
        db.run_migrations();
        assert!(db.table_exists("migrations"));
        drop(db);

        remove_file("test.db").unwrap();
    }
}
