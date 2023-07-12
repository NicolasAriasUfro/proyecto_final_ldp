use crate::controlador::manipular_info::info_almacenada::Entrada;
use crate::controlador::*;
use crate::controller_sql;
use dialoguer::{
    console::Term, theme::ColorfulTheme, Confirm, FuzzySelect, Input, Password, Select,
};
use manipular_info::info_almacenada::*;
use std::slice::ChunksExact;
use crate::controlador::manipular_info::crypto_base::Criptografia;

pub fn panel_loader() -> std::io::Result<()> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Bienvenido, presione enter para logear\nesc para salir")
        .item("ingresar")
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            if index == 0 {
                login()
            }
        }
        None => {
            println!("Saliendo");
            std::process::exit(0);
        }
    }
    Ok(())
}
fn login() {
    let contraseña_maestra = "12345678".to_string(); //for testing

    let password = Password::new()
        .with_prompt("Ingrese contraseña maestra:")
        .interact()
        .expect("Error al solicitar la contraseña, contraseña incorrecta");
}

// panel para crear la contraseña si la base de datos no existe, luego almacenarla en la base de datos y finalmente
pub fn panel_register() {
    let contra_maestra = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Bienvenido a el mejor gestor de contraseñas en rust
  Primero, debes generar una contraseña maestra
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
        println!("Creacion exitosa");
        panel_loader();
        
}

//panel principal de la aplicación, muestra todas las cuentas almacenadas con su titulo (si hay), user, url(si hay) y password
pub fn panel_main() {
    loop {
        seleccionar();
    }
}

fn sort_by_title() {
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
                vista_for_create()?
            }
            if index == 2 {
                vista_for_delete()?
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
    let llave_temporal: [u8; 32] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
    let cifrador_temporal = Criptografia::new(&llave_temporal);
    let mut lista_cuentas: Vec<Entrada> = controller_sql::listar_cuentas(&cifrador_temporal).unwrap();
    for i in 0..lista_cuentas.len() {
        let string_pusher_2 = format!(
            "{:<8} {:<8} {:<8} {:<8} {:<8} {:<8}",
            lista_cuentas[i].id,
            lista_cuentas[i].titulo.clone(),
            lista_cuentas[i].nombre_usuario,
            lista_cuentas[i].contrasena,
            lista_cuentas[i].fecha_creacion,
            lista_cuentas[i].url.clone()
        );
        cuentas_con_formato.push(string_pusher_2);
    }

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Id       Titulo     Usuario    Contraseña fecha      Url     ")
        .items(&cuentas_con_formato)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            cuenta_detallada(index);
        }
        None => {
            println!("Regresando")
        }
    }
    Ok(())
}

fn vista_for_create()-> std::io::Result<()>{
    let title: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("ingresa el titulo (presione enter))")
        .default("".to_string())
        .interact_text()
        .unwrap();
    let user: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("ingresa el user (obligatorio) (presione enter))")
        .interact_text()
        .unwrap();
    let url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("ingresa la url (presione enter))")
        .default("".to_string())
        .interact_text()
        .unwrap();
    let password = password_validator();
    let cuenta_a_subir = Entrada::new_creado(title, user, password, url);
    let llave_temporal: [u8; 32] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
    let cifrador_temporal = Criptografia::new(&llave_temporal);
    controller_sql::agregar_cuenta(&cuenta_a_subir,&cifrador_temporal);
    Ok(())
}

fn vista_for_delete() -> std::io::Result<()> {
    let default_choice_for_sort = false;

    let mut cuentas_con_formato = Vec::new();
    let llave_temporal: [u8; 32] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
    let cifrador_temporal = Criptografia::new(&llave_temporal);
    let mut lista_cuentas: Vec<Entrada> = controller_sql::listar_cuentas(&cifrador_temporal).unwrap();
    for i in 0..lista_cuentas.len() {
        let mut string_pusher_2 = format!(
            "{:<8} {:<8} {:<8} {:<8} {:<8} {:<8}",
            lista_cuentas[i].id,
            lista_cuentas[i].titulo.clone(),
            lista_cuentas[i].nombre_usuario,
            lista_cuentas[i].contrasena,
            lista_cuentas[i].fecha_creacion,
            lista_cuentas[i].url.clone()
        );
        cuentas_con_formato.push(string_pusher_2);
    }


    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Id       Titulo     Usuario    Contraseña fecha      Url     ")
        .items(&cuentas_con_formato)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            if Confirm::new()
                .with_prompt("Do you want to continue?")
                .interact()?
            {
                println!("Looks like you want to continue");
                controller_sql::eliminar_cuenta(&lista_cuentas[index])
                    .expect("no se ha podido eliminar la cuenta");
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


fn cuenta_detallada(index: usize) {
    
}
