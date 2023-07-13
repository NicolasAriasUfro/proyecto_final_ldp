use crate::controlador::manipular_info::comprobar_contra_maestra;
use crate::controlador::manipular_info::crypto_base::Criptografia;
use crate::controlador::manipular_info::info_almacenada::Entrada;
use crate::controlador::manipular_info::iniciar_base_de_datos_existente;
use crate::controlador::*;
use crate::controller_sql::recuperar_datos_master;
use crate::controller_sql::set_database;
use crate::{clipboard_generic, controller_sql};
use chrono::TimeZone;
use chrono::Utc;
use dialoguer::{
    console::Term, theme::ColorfulTheme, Confirm, FuzzySelect, Input, Password, Select,
};
use crate::controlador::manipular_info::generador_contra::generar_contra;
use crate::controlador::manipular_info::generador_contra::TipoContra::{ALFABETICO, ALFANUMERICO, ALFANUMERICOEXT, NUMERICO};

pub fn panel_loader() -> std::io::Result<()> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Bienvenido, presione enter para logear\n  esc para salir")
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
        println!("Contraseña incorrecta")
    }
}

pub fn panel_register() {
    let contra_maestra = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Bienvenido a el mejor gestor de contraseñas en rust
  Instrucciones:
  Para manejar el programa use las teclas enter para confirmar
  flecha arriba/abajo: para navegar por el menu
  Volver atras: esc
  A continuación, crea tu contraseña maestra
  
  AVISO: SI OLVIDAS ESTA CONTRASEÑA, NO PODRAS RECUPERAR TU BASE DE DATOS
  ",
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
        "Crear contraseña aleatoria"
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
                contrasena_aleatoria()?
            }
        }
        None => {
            println!("Saliendo");
            std::process::exit(0);
        }
    }

    Ok(())
}


fn vista_for_selection(cifrador: &Criptografia) -> std::io::Result<()> {
    let mut cuentas_con_formato = Vec::new();

    let lista_cuentas: Vec<Entrada> = controller_sql::listar_cuentas(&cifrador).unwrap();
    for i in 0..lista_cuentas.len() {
        let datetime=Utc.timestamp_opt(lista_cuentas[i].fecha_creacion as i64,0).unwrap();
        let string_pusher_2 = format!(
            "{:<4}|{:<13}|{:<20}|{:<16}|{:<10}|{:<8}",
            lista_cuentas[i].id,
            lista_cuentas[i].titulo.clone(),
            lista_cuentas[i].nombre_usuario,
            "********", /*lista_cuentas[i].contrasena*/
            datetime.to_rfc3339()[0..10].to_owned(),
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
        let datetime=Utc.timestamp_opt(lista_cuentas[i].fecha_creacion as i64,0).unwrap();
        let string_pusher_2 = format!(
            "{:<4}|{:<13}|{:<20}|{:<16}|{:<10}|{:<8}",
            lista_cuentas[i].id,
            lista_cuentas[i].titulo.clone(),
            lista_cuentas[i].nombre_usuario,
            lista_cuentas[i].contrasena,
            datetime.to_rfc3339()[0..10].to_owned(),
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
    let datetime=Utc.timestamp_opt(cuenta.fecha_creacion as i64,0).unwrap();


    let string_pusher_2 = format!(
        "{:<4}|{:<13}|{:<20}|{:<16}|{:<10}|{:<8}",
        &cuenta.id,
        &cuenta.titulo.clone(),
        &cuenta.nombre_usuario,
        &cuenta.contrasena,
        datetime.to_rfc3339()[0..10].to_owned(),
        &cuenta.url.clone()
    );

    cuenta_con_formato.push(string_pusher_2);
    clipboard_generic::copiar_al_portapapeles(&cuenta.contrasena);

    let _selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Id  |Titulo            |Usuario             |Contraseña      |Fecha     | Url                  ")
        .items(&cuenta_con_formato)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    Ok(())
}
fn contrasena_aleatoria() -> std::io::Result<()> {
    let opciones = vec!["Alfabética","Numérica","Alfanumérica","Con Carácteres Especiales"];// todo: cambiar el nombre de full

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Indique el tipo de contraseña requerido")
        .items(&opciones)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    let mut contrasena_generada= String::from("");
    match selection {
        Some(index) => {
            if index == 0 {
                contrasena_generada = generar_contra(ALFABETICO, 10)
            }
            if index == 1 {
                contrasena_generada = generar_contra(NUMERICO, 10)
            }
            if index == 2 {
                contrasena_generada = generar_contra(ALFANUMERICO, 10)
            }
            if index == 3 {
                contrasena_generada = generar_contra(ALFANUMERICOEXT, 10)
            }
        }
        None => {
            println!("Regresando")
        }
    }
    clipboard_generic::copiar_al_portapapeles(&contrasena_generada);
    let _texto: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(
            &contrasena_generada,
        )
        .default("".to_string())
        .interact_text()
        .unwrap();
    Ok(())
}