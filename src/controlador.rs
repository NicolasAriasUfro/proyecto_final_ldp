use dialoguer::Password;

use self::manipular_info::crypto_base::{hash_contra_maestra, crear_llave};
pub mod manipular_info;
use crate::controller_sql::*;
use crate::panel::{panel_loader, panel_register};
use std::fs::File;
use std::io;

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


pub fn load_app() {
    if existe_la_base_de_datos(){
        loop{
            panel_loader().unwrap();
        }
    }else{
        panel_register();
        
    }
}

pub fn password_validator() -> String {
    let password: String = Password::new()
        .with_prompt("Enter password")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.len() > 7 {
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

pub fn setear_llave_maestra(contra: String) -> std::io::Result<()>{
    let mut sal=[0u8;16];
    let mut llave=[0u8;32];
    let hash_contra:Vec<u8>;
    crear_llave(&contra.to_owned(), &mut llave, &mut sal);
    hash_contra=hash_contra_maestra(contra.as_bytes(), &sal);
    agregar_master(&hash_contra, &sal).unwrap();
    Ok(())
}