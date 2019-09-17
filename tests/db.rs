use backupd::db::*;
use std::fs::remove_file;

#[test]
fn open_new_db() {
    let mut db = DB::new("test.db", "test").unwrap();
    assert!(!db.table_exists("migrations"));
    db.run_migrations().unwrap();
    assert!(db.table_exists("migrations"));
    drop(db);

    remove_file("test.db").unwrap();
}

