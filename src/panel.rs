use std::slice::ChunksExact;

use dialoguer::{console::Term, theme::ColorfulTheme, FuzzySelect, Password, Select, Input, Confirm};

use crate::controlador::*;
use manipular_info::info_almacenada::*;

use crate::controlador::manipular_info::info_almacenada::Entrada;
use crate::controller_sql;



pub fn panel_loader(){

}
pub fn panel_login() {
    let contraseña_maestra = "12345678".to_string(); //for testing

    let password = Password::new()
        .with_prompt("Ingrese contraseña maestra:")
        .interact()
        .expect("Error al solicitar la contraseña, contraseña incorrecta");
}

// panel para crear la contraseña si la base de datos no existe, luego almacenarla en la base de datos y finalmente
pub fn panel_register() {
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Bienvenido a el mejor gestor de contraseñas en rust\n
                      Primero, debes generar una contraseña maestra\n
                      AVISO: SI OLVIDAS ESTA CONTRASEÑA, NO PODRAS RECUPERAR TU BASE DE DATOS")
        .with_confirmation("Repite la contraseña", "Error: las contraseñas no coinciden.")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.len() > 7 {
                Ok(())
            } else {
                Err("La contraseña debe contener a lo menos ocho caracteres")
            }
        })
        .interact()
        .unwrap();
        
}

//panel principal de la aplicación, muestra todas las cuentas almacenadas con su titulo (si hay), user, url(si hay) y password
pub fn panel_main() {
    loop {
        seleccionar();
    }

    //el panel main debe manejar alguna estructura de datos que contenga las cuentas, este obviamente debe ser poblado mediante instrucciones
    //SELECT , luego se recorre toda la estructura de datos y se imprimen las cuentas con su numero
    // el panel en la zona inferior debe manejar los siguientes comandos:
    // debe haber un limite de caracteres por mostrar en el panel, con 8 basta

    // FUNCIONALIDADES:
    // 1. Crear una nueva cuenta
    // 2. Borrar una cuenta
    // 3. Listar todas las cuentas por nombre/fecha (cambia con cada seleccion)
    // 4. Salir
    // 5. poder elegir una cuenta para mostrar su información
}

fn sort_by_title() {
    
    
    
    
    todo!()
}

pub fn seleccionar() -> std::io::Result<()> {
    let items = vec!["Ver cuentas", "Crear cuenta nueva", "Borrar cuenta", "instrucciones"];
    println!("para salir presione esc");
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Menu")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            if index == 0 {
                vista_for_selection()?
            }
            if index == 1 {
                vista_for_create()
            }
            if index == 2 {
                vista_for_delete()?
            }
            if index == 4{
                instrucciones()
            }
        }
        None => {
            println!("Saliendo");
            std::process::exit(0);
        }
    }

    Ok(())
}

fn instrucciones() {
    let texto: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Para manejar el programa use las teclas enter y barra espaciadora\n
                  Volver atras: esc")
    .default("".to_string())
    .interact_text()
    .unwrap();
}

fn vista_for_selection() -> std::io::Result<()> {
    let default_choice_for_sort = false;


    let mut cuentas_con_formato = Vec::new();
    let mut lista_cuentas:Vec<Entrada> = controller_sql::listar_cuentas().unwrap();
    for i in 0..lista_cuentas.len(){
        let mut string_pusher_2 = format!("{:<8} {:<8} {:<8} {:<8} {:<8} {:<8}",
                                      lista_cuentas[i].id,
                                      lista_cuentas[i].titulo.clone().unwrap(),
                                      lista_cuentas[i].nombre_usuario,
                                      lista_cuentas[i].contrasena,
                                      lista_cuentas[i].fecha_creacion,
                                      lista_cuentas[i].url.clone().unwrap());
        cuentas_con_formato.push(string_pusher_2);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Id       Titulo     Usuario    Contraseña fecha      Url     ")
        .items(&cuentas_con_formato)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            println!("todo");
        }
        None => {
            println!("Regresando")
        }
    }
    Ok(())
}

fn vista_for_create() {
    let mail: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("ingresa el titulo (presione enter))")
        .default("".to_string())
        .interact_text()
        .unwrap();

}

fn vista_for_delete() -> std::io::Result<()>{
    let default_choice_for_sort = false;


    let mut cuentas_con_formato = Vec::new();
    let mut lista_cuentas:Vec<Entrada> = controller_sql::listar_cuentas().unwrap();
    for i in 0..lista_cuentas.len(){
        let mut string_pusher_2 = format!("{:<8} {:<8} {:<8} {:<8} {:<8} {:<8}",
                                          lista_cuentas[i].id,
                                          lista_cuentas[i].titulo.clone().unwrap(),
                                          lista_cuentas[i].nombre_usuario,
                                          lista_cuentas[i].contrasena,
                                          lista_cuentas[i].fecha_creacion,
                                          lista_cuentas[i].url.clone().unwrap());
        cuentas_con_formato.push(string_pusher_2);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Id       Titulo     Usuario    Contraseña fecha      Url     ")
        .items(&cuentas_con_formato)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {

            if Confirm::new().with_prompt("Do you want to continue?").interact()? {
                println!("Looks like you want to continue");
                controller_sql::eliminar_cuenta(&lista_cuentas[index]).expect("no se ha podido eliminar la cuenta");
            } else {
                println!("nevermind then :(");
            }

        }
        None => {
            println!("Regresando")
        }
    }
    Ok(())
}



fn vista_for_update() {
    todo!()
}

// este panel debe mostrar el contenido de la contraseña una vez se escoja una

fn vista_for_contenido(cuenta: String) {
    todo!()



    //esta funcion muestra el contenido de la cuenta
    //debe permitir volver atras,
    //formato de ejemplo:
    //cuenta
    //Title:  // user  // url // password(*******)
    //volver atras: A mostrar contraseña: D Copiar contraseña: C

    /*
    Ejemplo de muestra de contraseña
    //Title:  // user  // url // password(12345678)
    //volver atras: A Ocultar contraseña: D Copiar contraseña: C
     */
}
