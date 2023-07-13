use crate::controlador::manipular_info::comprobar_contra_maestra;
use crate::controlador::manipular_info::crypto_base::Criptografia;
use crate::controlador::manipular_info::info_almacenada::Entrada;
use crate::controlador::manipular_info::iniciar_base_de_datos_existente;
use crate::controlador::*;
use crate::controller_sql::recuperar_datos_master;
use crate::controller_sql::set_database;
use crate::{clipboard_generic, controller_sql};
use dialoguer::{
    console::Term, theme::ColorfulTheme, Confirm, FuzzySelect, Input, Password, Select,
};

/*
use manipular_info::info_almacenada::*;
use std::slice::ChunksExact;
 */

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
    let contraseña_maestra = recuperar_datos_master().unwrap();
    let sal = &contraseña_maestra.1;
    let hash_almacenado = &contraseña_maestra.0;

    let password = Password::new()
        .with_prompt("Ingrese contraseña maestra")
        .interact()
        .expect("Error al solicitar la contraseña, contraseña incorrecta");
    if comprobar_contra_maestra(&password, sal, hash_almacenado) {
        let cifrador = iniciar_base_de_datos_existente(&password, &sal);
        panel_main(cifrador);
    } else {
        println!("error")
    }
}

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

    set_database().unwrap();
    setear_llave_maestra(contra_maestra).unwrap();
    println!("Creacion exitosa");
    panel_loader().unwrap();
}

//main del panel
pub fn panel_main(cifrador: Criptografia) {
    loop {
        seleccionar(&cifrador).unwrap();
    }
}

pub fn seleccionar(cifrador: &Criptografia) -> std::io::Result<()> {
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
                vista_for_selection(&cifrador)?
            }
            if index == 1 {
                vista_for_create(&cifrador)?
            }
            if index == 2 {
                vista_for_delete(&cifrador)?
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
            "Para manejar el programa use las teclas enter para confirmar
  flecha arriba/abajo: para navegar por el menu
  Volver atras: esc
  para salir de este menu, presione enter",
        )
        .default("".to_string())
        .interact_text()
        .unwrap();
}

fn vista_for_selection(cifrador: &Criptografia) -> std::io::Result<()> {
    let mut cuentas_con_formato = Vec::new();

    let lista_cuentas: Vec<Entrada> = controller_sql::listar_cuentas(&cifrador).unwrap();
    for i in 0..lista_cuentas.len() {
        let string_pusher_2 = format!(
            "{:<4}|{:<13}|{:<20}|{:<16}|{:<10}|{:<8}",
            lista_cuentas[i].id,
            lista_cuentas[i].titulo.clone(),
            lista_cuentas[i].nombre_usuario,
            "********", /*lista_cuentas[i].contrasena*/
            lista_cuentas[i].fecha_creacion,
            lista_cuentas[i].url.clone()
        );
        cuentas_con_formato.push(string_pusher_2);
    }

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Id  |Titulo        |Usuario             |Contraseña      |Fecha     | Url                  ")
        .items(&cuentas_con_formato)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            cuenta_detallada(&lista_cuentas[index]).expect("no se puede seleccionar la cuenta");
        }
        None => {
            println!("Regresando")
        }
    }
    Ok(())
}

fn vista_for_create(cifrador: &Criptografia) -> std::io::Result<()> {
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
    controller_sql::agregar_cuenta(&cuenta_a_subir, &cifrador).unwrap();
    println!("cuenta creada exitosamente");
    Ok(())
}

fn vista_for_delete(cifrador: &Criptografia) -> std::io::Result<()> {
    let mut cuentas_con_formato = Vec::new();

    let lista_cuentas: Vec<Entrada> = controller_sql::listar_cuentas(&cifrador).unwrap();
    for i in 0..lista_cuentas.len() {
        let string_pusher_2 = format!(
            "{:<4}|{:<12}|{:<20}|{:<16}|{:<10}|{:<8}",
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
        .with_prompt("Id  |Titulo      |Usuario             |Contraseña      |Fecha     | Url                  ")
        .items(&cuentas_con_formato)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            if Confirm::new()
            .with_prompt("¿quieres borrar la contraseña?")
            .interact()? {
                controller_sql::eliminar_cuenta(&lista_cuentas[index])
                    .expect("no se ha podido eliminar la cuenta");

                println!("Cuenta eliminada");
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

fn cuenta_detallada(cuenta: &Entrada) -> std::io::Result<()> {
    let mut cuenta_con_formato = Vec::new();

    let string_pusher_2 = format!(
        "{:<4}|{:<12}|{:<20}|{:<16}|{:<10}|{:<8}",
        &cuenta.id,
        &cuenta.titulo.clone(),
        &cuenta.nombre_usuario,
        &cuenta.contrasena,
        &cuenta.fecha_creacion,
        &cuenta.url.clone()
    );

    cuenta_con_formato.push(string_pusher_2);
    clipboard_generic::copiar_al_portapapeles(&cuenta.contrasena);

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Id  |Titulo            |Usuario             |Contraseña      |Fecha     | Url                  ")
        .items(&cuenta_con_formato)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    Ok(())
}
