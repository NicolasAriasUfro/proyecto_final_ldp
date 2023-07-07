use rusqlite::{params, Connection, Result};
use crate::manipular_info::info_almacenada::*;

//TODO: agregar tabla que verifique la contraseña usada para entrar al programa.
pub fn set_database(){
    //crear la tabla de passwords
    let conn:Connection = Connection::open("database.db")?;

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
}
/*
pub fn agregar_cuenta(cuenta:&Entrada){
    let conn:Connection = Connection::open("database.db")?;

    let temp_cuenta = cuenta.clone();

    //Todo: obtener los parametros de la cuenta con getters
    conn.execute(
        "INSERT INTO cuentas (title, hash_password,user_name) VALUES (?1, ?2, ?3)",
        (&temp_cuenta.id, &temp_cuenta.contrasena, &temp_cuenta.nombre_usuario)
    )?;

}
*/












