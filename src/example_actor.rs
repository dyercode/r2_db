use crate::db_interactions;
use actix::{Actor, Context, Handler, Message};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Message)]
pub struct Bind {
    pub pool: Pool<SqliteConnectionManager>,
}

#[derive(Message)]
pub struct Write {
    pub name: String,
    pub value: String,
}

pub struct DbAcceptor {
    pub pool: Option<Pool<SqliteConnectionManager>>,
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
        if let Some(conn) = &self.pool {
            db_interactions::insert_config(conn, &msg.name, &msg.value);
        }
    }
}
