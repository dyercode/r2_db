use actix::{Actor, Addr, Context, Handler, Message, System};
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

    let acceptor1 = DbAcceptor { pool: None };
    let acceptor2 = DbAcceptor { pool: None };
    let config_db = ConfigDb { pool: pool.clone() };

    let system = System::new("yas");
    let addr1 = acceptor1.start();
    let addr2 = acceptor2.start();
    bind_and_write(addr1, config_db.clone(), "key", "not_key");
    bind_and_write(addr2, config_db, "key2", "2nd_not_key");

    System::current().stop();
    system.run().unwrap_or_else(|_| panic!("can't even system"));
}

fn bind_and_write(addr: Addr<DbAcceptor>, config_db: ConfigDb, name: &str, value: &str) {
    addr.try_send(Bind { config_db }).expect("bind failed");
    addr.try_send(Write {
        name: name.to_string(),
        value: value.to_string(),
    })
    .expect("write failed");
}

#[derive(Clone)]
struct ConfigDb {
    pool: r2d2::Pool<SqliteConnectionManager>,
}

impl ConfigDb {
    fn insert_config(&self, name: &str, value: &str) {
        if let Ok(conn) = self.pool.get() {
            conn.execute(
                "insert into config (name, value) values (?,?)",
                &[name, value],
            )
            .unwrap_or_else(|_| panic!("can't write omg"));
        }
    }
}

#[derive(Message)]
struct Bind {
    config_db: ConfigDb,
}

#[derive(Message)]
struct Write {
    name: String,
    value: String,
}

struct DbAcceptor {
    pool: Option<ConfigDb>,
}

impl Actor for DbAcceptor {
    type Context = Context<Self>;
}

impl Handler<Bind> for DbAcceptor {
    type Result = ();

    fn handle(&mut self, msg: Bind, _ctx: &mut Self::Context) -> Self::Result {
        self.pool = Some(msg.config_db);
    }
}

impl Handler<Write> for DbAcceptor {
    type Result = ();

    fn handle(&mut self, msg: Write, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(db) = &self.pool {
            db.insert_config(&msg.name, &msg.value);
        }
    }
}
