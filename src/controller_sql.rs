use rusqlite::{params, Connection, Result};
pub fn set_database(){
    let conn = Connection::open("database.db")?;
    //crear la tabla de passwords
    conn.execute("create table cuentas IF NOT EXISTS(
    id            integer primary key autoincrement ,
    title         TEXT,
    hash_password TEXT NOT NULL,
    user_name     TEXT NOT NULL,
    url           TEXT,
    desc_password TEXT NOT NULL
    )",
    (),
    )?;
    //TODO: agregar tabla que verifique la contraseña usada para entrar al programa.
}
//pub fn agregar_cuenta($cuenta:cuenta)












