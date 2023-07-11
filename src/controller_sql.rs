use rusqlite::{params, Connection, Result};
use crate::controlador::manipular_info::info_almacenada::*;

pub fn set_database() -> Result<()> {
    let conn = Connection::open("database.db")?;
    //crear la tabla de cuentas
    conn.execute("drop table cuentas",params![])?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cuentas (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            title         TEXT,
            user_name     TEXT NOT NULL,
            hash_password TEXT NOT NULL,
            nonce         TEXT NOT NULL,
            fecha         INTEGER,
            url           TEXT
        )",
        params![],
    )?;

    /*
    conn.execute(
        " ALTER TABLE cuentas ADD nonce TEXT",
        params![])?;
    */
    /*
    conn.execute(
        " ALTER TABLE cuentas ADD fecha TEXT",
        params![])?;
    */
    Ok(())
}
///Recibe una cuenta que ya tenga asignado su id,
/// y actualiza todos los demás campos en la base de datos.
fn actualizar_cuenta(cuenta: &Entrada) -> Result<()> {
    let conn = Connection::open("database.db")?;
    conn.execute(
        "UPDATE cuentas SET
        title = ?2,
        username = ?3,
        hash_password = ?4,
        nonce = ?5,
        fecha = ?6,
        url = ?7,
        WHERE id = ?1",
        params![&cuenta.id, &cuenta.titulo, &cuenta.nombre_usuario,&cuenta.contrasena,
        &cuenta.nonce, &cuenta.fecha_creacion,&cuenta.url],
    )?;
    Ok(())
}
///Recibe una cuenta y según su id elimina la cuenta de la base de datos.
/// el id es asignado cuando se crea la Entrada a partir de la base de datos
pub fn eliminar_cuenta(cuenta:&Entrada) -> Result<()>{
    let conn = Connection::open("database.db")?;
    conn.execute("DELETE FROM cuentas WHERE id = ?1", params![cuenta.id])?;
    Ok(())

}

/// Ingresa una cuenta a la base de datos, donde se le asigna un id automáticamente
/// La cuenta debe tener cifrada el atributo contraseña
pub fn agregar_cuenta(cuenta:&Entrada) -> Result<()>{

    let conn = Connection::open("database.db")?;
    conn.execute(
        "INSERT INTO cuentas (title,user_name,hash_password,nonce,fecha,url)
            Values(?1,?2,?3,?4,?5,?6)",
        (&cuenta.titulo,&cuenta.nombre_usuario,&cuenta.contrasena,&cuenta.nonce,&cuenta.fecha_creacion,&cuenta.url)
    )?;
    Ok(())
}
pub fn listar_cuentas()-> Result<()>{
    let conn = Connection::open("database.db")?;
    let mut stmt = conn.prepare(
        "SELECT id, title, user_name, hash_password, nonce, fecha, url from cuentas")?;

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
