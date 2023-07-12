use std::{env, process::exit, hash};

use base64::{engine::general_purpose, Engine};
use controlador::manipular_info::crypto_base::crear_nonce;
use controller_sql::{agregar_master, recuperar_datos_master};

use crate::{controlador::manipular_info::{crypto_base::{hash_contra_maestra, crear_llave, derivar_llave}, info_almacenada::{recrear_nonce}, iniciar_nueva_base_de_datos}, controller_sql::set_database};
use crate::controlador::manipular_info::info_almacenada::Entrada;


mod clipboard_generic;
mod panel;
mod controller_sql;
mod controlador;

fn main() {

    for argument in env::args(){
        match argument.as_str(){
            "--test"=>{
                tests();
                exit(0);
            },
            _=>{}

        }
    }

    //preconfigrar y poblar la base de datos
    controller_sql::set_database();
    //poblar_base_de_datos();
    //ejecutar el panel
    test_panel();



    /*
    controller_sql::agregar_cuenta(&Entrada::new_creado("".to_owned(),"twitter".to_owned(),"12345678".to_owned(),"".to_owned())).unwrap();
    comprobar_contra();
    let clave="contra".to_owned();
    let mut sal=[0u8;16];
    let mut llave=[0u8;32];
    crear_llave(&clave, &mut llave, &mut sal);
    let hash_clave=hash_contra_maestra(&clave.as_bytes(), &sal);
    println!("key={:?}\n key_hash={:?}",llave,hash_contra_maestra(&clave.as_bytes(), &sal));
    assert_ne!(hash_contra_maestra(&clave.as_bytes(), &sal),llave);
    assert_eq!(hash_clave,hash_contra_maestra(&clave.as_bytes(), &sal));

     */

}


fn tests(){
    println!("testing.");
    let clave="contra".to_owned();
    let mut sal=[0u8;16];
    let mut llave=[0u8;32];
    crear_llave(&clave, &mut llave, &mut sal);
    //let mut sal_loging=sal.clone();
    //sal_loging[2]=0u8;
    println!("key={:?}\n key_hash={:?}",llave,hash_contra_maestra(&llave, &sal));
    assert_ne!(hash_contra_maestra(&llave, &sal),llave);
    reconstruir_nonce();
    iniciar_nueva_base_de_datos(&"abcdefg".to_owned());
    comprobar_contra();
}

fn reconstruir_nonce(){
    let nonce=crear_nonce();
    println!("nonce: {:?}",nonce);

    let nonce_b64=general_purpose::STANDARD_NO_PAD.encode(&nonce);
    let decoded=recrear_nonce(nonce_b64);
    println!("decoded: {:?}",decoded);
    assert_eq!(nonce,decoded);
}

fn test_panel(){
    //panel::panel_main();
    controlador::load_app();
}

fn comprobar_contra(){
    set_database().unwrap();
    println!("comprobando contra");
    let contra="contra_bkn_123";
    let mut sal=[0u8;16];
    let mut llave=[0u8;32];
    let hash_contra:Vec<u8>;
    crear_llave(&contra.to_owned(), &mut llave, &mut sal);
    hash_contra=hash_contra_maestra(contra.as_bytes(), &sal);
    agregar_master(&hash_contra, &sal).unwrap();
    let par=recuperar_datos_master().unwrap();
    assert_eq!(hash_contra,par.0);
    assert_eq!(sal,par.1);
    let clave_again="contra_bkn_123";
    let llave_recreada=derivar_llave(&clave_again.to_owned(), &par.1);
    assert_eq!(llave_recreada,llave);
    println!("llave original: {:?}\n llave recreada: {:?}",llave,llave_recreada);
}
fn poblar_base_de_datos(){
    let cuenta_1:Entrada = Entrada::new_creado(
        String::from("Codeforces"),
        "nico_arias".parse().unwrap(),
        "asd24355##".parse().unwrap(),
        String::from("https://codeforces.com/")
    );
    let cuenta_2:Entrada = Entrada::new_creado(
        String::from("youtube"),
        "juega_german".parse().unwrap(),
        "3333444244".parse().unwrap(),
        String::from("https://youtube.com")
    );
    let cuenta_3:Entrada = Entrada::new_creado(
        String::from("facebook"),
        "el_brayan".parse().unwrap(),
        "Brayan_brayan".parse().unwrap(),
        String::from("https://facebook.com")
    );


/*
    controller_sql::agregar_cuenta(&cuenta_1);
    controller_sql::agregar_cuenta(&cuenta_2);
    controller_sql::agregar_cuenta(&cuenta_3);
*/

}