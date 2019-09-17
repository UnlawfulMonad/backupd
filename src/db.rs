use std::path::Path;
use rusqlite::Connection;

use regex::Regex;

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new<P: AsRef<Path>>(conn: Connection) -> DB {
        DB { conn }
    }

    pub fn set_key(&self, key: &str) {
        let reg = Regex::new("^[^[:space:]]+$").unwrap();
        assert!(reg.is_match(key));

        let cmd = format!("PRAGMA key = '{}';", key);
        self.conn.execute_batch(&cmd).unwrap();
    }
}
