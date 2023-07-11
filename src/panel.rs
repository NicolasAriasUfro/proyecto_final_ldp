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
    loop{
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
    let items = vec![
        "Ver cuentas",
        "Crear cuenta nueva",
        "Borrar cuenta",
    ];
    println!("para salir presione esc");
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Menu")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;


    match selection {
        Some(index) => {
            if index == 0 {
                vista_for_selection();
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


    let cuentas = vec![
        "Fortnite              daddyisues             null            23/07/2023",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Titulo                    usuario                 Contraseña                  Url                 Fecha")
        .items(&cuentas)
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

    /*
    Ejemplo de vista del panel
    CUENTAS:
    1 titulo   user         url                       password
    2 google   juan         https://www.google.com    ********
    3          a@gmail.com                            ********

    crear nueva cuenta: q borrar una cuenta:w salir: e
     */
}

fn vista_for_delete() {}

// este panel debe mostrar el contenido de la contraseña una vez se escoja una
pub fn panel_contenido() {
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
