use rusqlite::{params, Connection, Result};
use crate::controlador::manipular_info::info_almacenada::Entrada;

pub fn set_database() -> Result<()> {
    let conn = Connection::open("database.db")?;
    //crear la tabla de cuentas
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cuentas (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            title         TEXT,
            user_name     TEXT NOT NULL,
            hash_password TEXT NOT NULL,
            url           TEXT,
            desc_password TEXT NOT NULL
        )",
        params![],
    )?;
    Ok(())
    //TODO: agregar tabla que verifique la contraseña usada para entrar al programa.
}

pub fn create_column() {}

pub fn select_column() {
    todo!();
}



pub fn sql_instruction_get_password() {
    
}

pub fn update_column() {}

pub fn delete_column() {}


pub fn agregar_cuenta(cuenta:&Entrada) -> Result<()>{
    let conn = Connection::open("database.db")?;
    conn.execute(
        "INSERT INTO cuentas (title,user_name,hash_password,url)
            Values(?1,?2,V3)",
        params![cuenta.titulo,cuenta.nombre_usuario,cuenta,cuenta.hash_password,cuenta.url]
    )?;
    Ok(())
}
