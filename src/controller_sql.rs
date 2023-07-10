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
            nonce         TEXT NOT NULL,
            url           TEXT,
            desc_password TEXT
        )",
        params![],
    )?;
    //TODO: agregar tabla que verifique la contraseña usada para entrar al programa.

    conn.execute(
        " ALTER TABLE cuentas ADD nonce TEXT",
        params![])?;
    conn.execute(
        " ALTER TABLE cuentas ADD fecha TEXT",
        params![])?;
    Ok(())
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
    println!("msg 3");

    let conn = Connection::open("database.db")?;
    conn.execute(
        "INSERT INTO cuentas (title,user_name,hash_password,nonce,url,fecha)
            Values(?1,?2,?3,?4,?5,?6)",
        (&cuenta.titulo,&cuenta.nombre_usuario,&cuenta.contrasena,&cuenta.nonce,&cuenta.url,&cuenta.fecha_creacion)
    )?;
    println!("msg 1");
    Ok(())
}
pub fn listar_cuentas()-> Result<()>{
    let conn = Connection::open("database.db")?;
    let mut stmt = conn.prepare(
        "SELECT id, title, user_name, hash_password, nonce, fecha, url, desc_password from cuentas")?;

    let cuentas_rows = stmt.query_map([],|row|{
        Ok(Entrada::new_desde_bd(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?
        )
        )
    })?;//ver formas de devolver cuentas_row para ser usada por panel
    for cuenta in cuentas_rows{
        let cuenta = cuenta?;
        println!("{:#?}",cuenta)
    }

    Ok(())

}
