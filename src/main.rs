mod db_interactions;
mod example_actor;

use crate::example_actor::{Bind, DbAcceptor, Write};
use actix::{Actor, Addr, System};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::NO_PARAMS;

fn main() {
    let manager = SqliteConnectionManager::file("file.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    pool.clone()
        .get()
        .unwrap()
        .execute(
            "create table if not exists config (name String, value String)",
            NO_PARAMS,
        )
        .unwrap();

    let acceptor1 = example_actor::DbAcceptor { pool: None };
    let acceptor2 = DbAcceptor { pool: None };

    let system = System::new("yas");
    let addr1 = acceptor1.start();
    let addr2 = acceptor2.start();
    bind_and_write(addr1, pool.clone(), "key", "not_key");
    bind_and_write(addr2, pool, "key2", "2nd_not_key");

    System::current().stop();
    system.run().unwrap_or_else(|_| panic!("can't even system"));
}

fn bind_and_write(
    addr: Addr<DbAcceptor>,
    pool: Pool<SqliteConnectionManager>,
    name: &str,
    value: &str,
) {
    addr.try_send(Bind { pool }).expect("bind failed");
    addr.try_send(Write {
        name: name.to_string(),
        value: value.to_string(),
    })
    .expect("write failed");
}
