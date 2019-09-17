use std::path::Path;
use rusqlite::Connection;

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new<P: AsRef<Path>>(path: P, key: &str) -> DB {
        let conn = Connection::open(path).expect("Failed to open DB");
        conn.pragma_update(None, "key", &key).unwrap();
        DB { conn }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn open_new_db() {
        let db = DB::new("test.db", "test");
        drop(db);

        remove_file("test.db").unwrap();
    }
}
