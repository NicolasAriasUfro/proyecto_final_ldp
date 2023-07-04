use rusqlite::{Connection,Result};
// playground by nico

pub fn playground() -> rusqlite::Result<()> {
    let conn  = Connection::open("cat.db");
    conn.execute(
        "create table if not exists cat_colors(
        id integer primary key,
        name text not null unique
        )",
        [],
    )?

}