use std::path::Path;
use rusqlite::{Connection, params};
use crate::error as e;
use log::info;

// The list of migrations. The tuple is in the form (name, sql).
//
// Used in `DB::run_migrations`.
const MIGRATIONS: &[(&str, &str)] = &[
    ("create initial schema", r"
CREATE TABLE files (
    id INTEGER PRIMARY KEY,
    path TEXT NOT NULL,
    filename TEXT,
    metadata TEXT NOT NULL DEFAULT '{}'
);
CREATE INDEX files_paths ON files(path, filename);

CREATE TABLE backups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date DATETIME NOT NULL,
    tag TEXT
);

CREATE TABLE blocks (
    hash BLOB PRIMARY KEY
);

CREATE TABLE files_blocks (
    block_order INTEGER,
    file_id INTEGER NOT NULL REFERENCES files(id),
    block_hash BLOB NOT NULL REFERENCES blocks(hash)
);
CREATE INDEX files_blocks_blocks_in_order ON files_blocks(file_id, block_order);
"),
];

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new<P: AsRef<Path>>(path: P, key: &str) -> e::Result<DB> {
        assert!(key.len() > 0);
        //const MMAP_SIZE: isize = 1024 * 1024 * 512;

        let conn = Connection::open(path)?;
        conn.pragma_update(None, "key", &key)?;
        //conn.pragma_update(None, "mmap_size", &MMAP_SIZE)?;
        Ok(DB { conn })
    }

    /**
     * Run all the migrations to get a node's tables up to date.
    */
    pub fn run_migrations(&mut self) -> e::Result<()> {
        const MIGRATIONS_TABLE: &str = "CREATE TABLE migrations (
                name TEXT NOT NULL,
                has_run BOOL NOT NULL
            )";
        if !self.table_exists("migrations") {
            info!("New database detected. Creating migrations table.");
            self.conn.execute_batch(MIGRATIONS_TABLE)?;
        }

        for (name, sql) in MIGRATIONS {
            let row: Result<String, _> = self.conn.query_row(
                "SELECT * FROM migrations WHERE has_run = true",
                params!(),
                |row| row.get(0),
            );

            if let Ok(_name) = row {
                continue;
            }

            info!("Running migration: {}", name);
            let tx = self.conn.transaction()?;
            tx.execute_batch(sql)?;
            tx.execute("INSERT INTO migrations(name, has_run) VALUES (?, ?)", params!(name, &true))?;
            tx.commit()?;
        }
        Ok(())
    }

    /**
     * Check if the given table exists.
     *
     * Uses `PRAGMA table_info(?)` and checks if data is returned.
    */
    pub fn table_exists(&self, name: &str) -> bool {
        let mut ret = false;
        self.conn.pragma(None, "table_info", &name, |_row| {
            ret = true;
            Ok(())
        }).unwrap();
        ret
    }

    pub fn add_block(&self, block_hash: &[u8]) -> e::Result<()> {
        self.conn.execute("INSERT INTO blocks(hash) VALUES (?) ON CONFLICT DO NOTHING", &[block_hash])?;
        Ok(())
    }
}
