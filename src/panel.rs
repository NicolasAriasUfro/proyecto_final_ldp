use std::slice::ChunksExact;

use dialoguer::{console::Term, theme::ColorfulTheme, FuzzySelect, Password, Select};

use crate::controlador::*;

pub fn panel_login() {
    let password = "12345678".to_string(); //for testing

    let password = Password::new()
        .with_prompt("Ingrese su contraseña:")
        .interact()
        .expect("Error al solicitar la contraseña, contraseña incorrecta");
}

// panel para crear la contraseña si la base de datos no existe, luego almacenarla en la base de datos y finalmente
pub fn panel_register() {
    // ingrese la contraseña maestra, esta debe tener como mínimo 8 caracteres.
    // ingrese la contraseña nuevamente.
    //
    // si esta ok println!("contraseña creada con exito");
    // se dirige al panel_main
    todo!()
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
    std::process::exit(0);
}

fn sort_by_title() {
    todo!()
}

pub fn seleccionar() -> std::io::Result<()> {
    let items = vec!["Ver cuentas", "Crear cuenta nueva", "Borrar cuenta"];
    println!("para salir presione esc");
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Menu")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            if index == 0 {
                vista_for_selection()?;
            }
            if index == 1 {
                println!("agua")
            }
            if index == 2 {
                println!("tierra")
            }
        }
        None => {
            println!("Saliendo");
            std::process::exit(0);
        }
    }

    Ok(())
}

fn vista_for_selection() -> std::io::Result<()> {
    let default_choice_for_sort = false;


    
    let mut cuentas = Vec::new();
    let id: String = "1".to_string();
    let title: String = "Fornite".to_string();
    let user: String = "abzassssdsdsds".to_string();
    let url: String = "www.epicgames.com".to_string();
    let password: String= "*********".to_string();
    let date: String = "23/07/2023".to_string();

    let string_pusher = format!("{:<2}   {:<8}          {:<8}      {:<8}   {:<8}           {:<8}  ", id, title, user, url, password, date);
    cuentas.push(string_pusher);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("id   Titulo            Usuario             Url                 Contraseña          fecha   \n\n  Para mas detalles selecione")
        .items(&cuentas)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            vista_for_contenido(cuentas[index].clone());
        }
        None => {
            println!("Regresando")
        }
    }
    Ok(())
}

fn vista_for_delete() {}

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
