use std::slice::ChunksExact;

use dialoguer::{console::Term, theme::ColorfulTheme, FuzzySelect, Input, Password, Select};

use crate::controlador::*;
use manipular_info::info_almacenada::*;

use crate::controlador::manipular_info::info_almacenada::Entrada;
use crate::controller_sql;

pub fn panel_loader() -> std::io::Result<()> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Bienvenido, presione enter para logear\nesc para salir")
        .item("ingresar")
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => if index == 0 {
            panel_login()
        },
        None => {
            println!("Saliendo");
            std::process::exit(0);
        }
    }
    Ok(())
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
        .with_prompt(
            "Bienvenido a el mejor gestor de contraseñas en rust\n
                      Primero, debes generar una contraseña maestra\n
                      AVISO: SI OLVIDAS ESTA CONTRASEÑA, NO PODRAS RECUPERAR TU BASE DE DATOS",
        )
        .with_confirmation(
            "Repite la contraseña",
            "Error: las contraseñas no coinciden.",
        )
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
}

fn sort_by_() {
    todo!()
}

pub fn seleccionar() -> std::io::Result<()> {
    let items = vec![
        "Ver cuentas",
        "Crear cuenta nueva",
        "Borrar cuenta",
        "instrucciones",
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
                vista_for_selection()?
            }
            if index == 1 {
                vista_for_create()
            }
            if index == 2 {
                vista_for_delete()
            }
            if index == 3 {
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
        .with_prompt(
            "Para manejar el programa use las teclas enter y barra espaciadora\n
                  Volver atras: esc",
        )
        .default("".to_string())
        .interact_text()
        .unwrap();

}

fn vista_for_selection() -> std::io::Result<()> {
    let default_choice_for_sort = false;

    let mut cuentas_con_formato = Vec::new();
    let mut lista_cuentas: Vec<Entrada> = controller_sql::listar_cuentas().unwrap();
    for i in 0..lista_cuentas.len() {
        let string_pusher_2 = format!(
            "{:<8} {:<8} {:<8} {:<8} {:<8} {:<8}",
            lista_cuentas[i].id,
            lista_cuentas[i].titulo.clone().unwrap(),
            lista_cuentas[i].nombre_usuario,
            lista_cuentas[i].contrasena,
            lista_cuentas[i].fecha_creacion,
            lista_cuentas[i].url.clone().unwrap()
        );
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

fn vista_for_delete() {
    todo!()
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
