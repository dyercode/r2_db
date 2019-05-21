use actix::{Actor, Context, Handler, Message, System};
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

    let acceptor = DbAcceptor { pool: None };

    let system = System::new("yas");
    let addr = acceptor.start();
    addr.try_send(Bind { pool: pool.clone() })
        .expect("bind message");
    addr.try_send(Write {
        name: "key".to_string(),
        value: "not_key".to_string(),
    })
    .expect("write message");
    System::current().stop();
    system.run().unwrap_or_else(|_| panic!("can't even system"));
}

struct DbAcceptor {
    pool: Option<r2d2::Pool<SqliteConnectionManager>>,
}

#[derive(Message)]
struct Bind {
    pool: r2d2::Pool<SqliteConnectionManager>,
}

#[derive(Message)]
struct Write {
    name: String,
    value: String,
}

impl Actor for DbAcceptor {
    type Context = Context<Self>;
}

impl Handler<Bind> for DbAcceptor {
    type Result = ();

    fn handle(&mut self, msg: Bind, _ctx: &mut Self::Context) -> Self::Result {
        self.pool = Some(msg.pool);
    }
}

impl Handler<Write> for DbAcceptor {
    type Result = ();

    fn handle(&mut self, msg: Write, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(db) = self.pool.clone() {
            if let Ok(conn) = db.get() {
                conn.execute(
                    "insert into config (name, value) values (?,?)",
                    &[&msg.name, &msg.value],
                )
                .unwrap_or_else(|_| panic!("can't write omg"));
            }
        }
    }
}
