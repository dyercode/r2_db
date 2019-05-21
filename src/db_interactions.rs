use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub fn insert_config(pool: &Pool<SqliteConnectionManager>, name: &str, value: &str) {
    if let Ok(conn) = pool.get() {
        conn.execute(
            "insert into config (name, value) values (?,?)",
            &[name, value],
        )
        .unwrap_or_else(|_| panic!("can't write omg"));
    };
}
