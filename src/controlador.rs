use dialoguer::Password;

use self::manipular_info::crypto_base::hash_contra_maestra;
pub mod manipular_info;
use crate::controller_sql;
use crate::panel::panel_loader;
use std::fs::File;

/* 

pub fn password_validator(password: &str) {//-> bool {
    let password: String = Password::new()
    .with_prompt("Enter password")
    .validate_with(|input: &String| -> Result<(), &str> {
        if input.len() > 8 {
            Ok(())
        } else {
            Err("Password must be longer than 8")
        }
    })
    .interact()
    .unwrap();
}
*/

pub fn load_app() ->    Result<(), io::Error> {
    match File::open("database.db"){
        Ok(file)=>{
            controller_sql::set_database();
            panel_loader();
        },
        Err(error) => Err(error),
    }
   
    

}
//funcion temporal para validar la contraseña
pub fn password_validator() -> String {
    let password: String = Password::new()
        .with_prompt("Enter password")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.len() > 8 {
                Ok(())
            } else {
                Err("Password must be longer than 8")
            }
        })
        .interact()
        .unwrap();
    password
}

pub fn validar_llave_maestra(contra_recibida:&[u8],sal:&[u8;16])->bool{
    //recuperar de la base de datos
    let hash_guardado:String=String::new();
    let hash_guardado=hash_guardado.as_bytes().to_owned();
    let hash_calculado=hash_contra_maestra(contra_recibida, sal);
    hash_guardado==hash_calculado
}


pub fn get_password(password: &str) {
    // Llama a la función de validación de llave maestra pasando la contraseña ingresada
    // let valid = validar_llave_maestra(password);
    /* 
    if valid {
        println!("Contraseña válida. Acceso permitido.");
    } else {
        println!("Contraseña inválida. Acceso denegado.");
    }
    */

    // TODO: esta funcion debe obtener la contraseña de la base de datos una vez se halla validado 
    // con la contraseña ingresada, debe dar una excepción si la contraseña es incorrecta.

}




