use rusqlite::{params, Connection, Result};
use crate::controlador::manipular_info::info_almacenada::*;

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
            Values(?1,?2,?3,?4)",
        (&cuenta.titulo,&cuenta.nombre_usuario,&cuenta.contrasena,&cuenta.url)
    )?;
    Ok(())
}
pub fn listar_cuentas()-> Result<()>{
    let conn = Connection::open("database.db")?;
    let mut stmt = conn.prepare(
        "SELECT id, title, user_name, hash_password, url, desc_password from cuentas")?;

    let cuentas_rows = stmt.query_map([],|row|{
        Ok(Entrada::new_desde_bd(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            [0,0,0,0,0,0,0,0,0,0,0,0], //Todo revisar el correcto funcionamiento del nonce
            5,
            row.get(4)?
        )
        )
    })?;//ver formas de devolver cuentas_row para ser usada por panel
    Ok(())

}
